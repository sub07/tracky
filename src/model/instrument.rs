use std::{fmt::Debug, iter, time::Duration};

use joy_vector::{vector, Vector};

use crate::audio::{
    frame::{Frame, StereoFrame},
    signal, synthesis, Pan, Volume,
};

use super::midi::C5_FREQ;

#[derive(Clone)]
pub enum Kind {
    Sine,
    Square,
    Sawtooth,
    Sample(signal::stereo::Owned),
}

impl Debug for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sine => write!(f, "Sine"),
            Self::Square => write!(f, "Square"),
            Self::Sawtooth => write!(f, "Sawtooth"),
            Self::Sample(_) => write!(f, "Sample"),
        }
    }
}

impl Kind {
    pub fn next_frame(
        &self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        frame_rate: f32,
    ) -> StereoFrame {
        match self {
            Kind::Sine => synthesis::sine_wave(freq, volume, pan, phase, frame_rate),
            Kind::Square => synthesis::square_wave(freq, volume, pan, phase, frame_rate),
            Kind::Sawtooth => synthesis::sawtooth_wave(freq, volume, pan, phase, frame_rate),
            Kind::Sample(signal) => {
                let Vector([l, r]) = signal
                    .as_ref()
                    .lerp_frame_at_duration(Duration::from_secs_f32(*phase / frame_rate))
                    .unwrap_or_default();

                *phase += freq / *C5_FREQ;

                let left_amp = volume.value() * pan.left_volume().value();
                let right_amp = volume.value() * pan.right_volume().value();
                vector!(l * left_amp, r * right_amp)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instrument {
    source: Kind, // TODO: Wrap in interpolator (for volume, and freq at least)
    pub volume: Volume,
}

impl From<Kind> for Instrument {
    fn from(value: Kind) -> Self {
        Instrument {
            source: value,
            volume: Volume::DEFAULT,
        }
    }
}

impl Instrument {
    pub fn next_frame(
        &self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        frame_rate: f32,
    ) -> StereoFrame {
        self.source.next_frame(freq, volume, pan, phase, frame_rate) * self.volume
    }

    pub fn collect_frame_in(
        &self,
        mut signal: signal::stereo::Mut,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
    ) {
        signal.fill(Frame::default());
        let frame_rate = signal.frame_rate;
        for (output, generated) in signal.iter_mut().zip(iter::repeat_with(|| {
            self.next_frame(freq, volume, pan, phase, frame_rate)
        })) {
            *output = generated;
        }
    }
}

pub const MAX_SLOT: usize = 64;

#[derive(Clone, Debug)]
pub struct Instruments {
    slots: [Option<Instrument>; MAX_SLOT],
}

impl Default for Instruments {
    fn default() -> Self {
        let mut slots = [const { None }; MAX_SLOT];
        slots[0] = Some(Instrument::from(Kind::Sine));
        slots[1] = Some(Instrument::from(Kind::Square));
        slots[2] = Some(Instrument::from(Kind::Sawtooth));
        slots[3] = Some(Instrument::from(Kind::Sample(
            signal::stereo::Owned::from_path("assets/stereo.wav").unwrap(),
        )));
        Self { slots }
    }
}

impl Instruments {
    pub fn get(&self, index: u8) -> Option<&Instrument> {
        self.slots.get(index as usize).and_then(Option::as_ref)
    }
}
