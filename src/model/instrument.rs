use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use joy_vector::{vector, Vector};

use crate::audio::{frame::StereoFrame, signal, synthesis, Pan, Volume};

use super::midi::C5_FREQ;

#[derive(Clone, Debug)]
pub enum Kind {
    Sine,
    Square,
    Sawtooth,
    Sample {
        name: String,
        signal: signal::stereo::Owned,
    },
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sine => write!(f, "Sine"),
            Self::Square => write!(f, "Square"),
            Self::Sawtooth => write!(f, "Sawtooth"),
            Self::Sample { name, .. } => write!(f, "{name}"),
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
            Kind::Sample { signal, .. } => {
                let Vector([l, r]) = signal
                    .as_ref()
                    .lerp_frame_at_duration(Duration::from_secs_f32(*phase))
                    .unwrap_or_default();

                *phase += freq / *C5_FREQ / frame_rate;

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
}

// Should be a u8 because an instrument index is represented by a a 2-digits hex number
pub const MAX_SLOT_COUNT: u8 = 64;

#[derive(Clone, Debug)]
pub struct Instruments {
    slots: [Option<Instrument>; MAX_SLOT_COUNT as usize],
    selected_index: u8,
}

impl Default for Instruments {
    fn default() -> Self {
        let mut slots = [const { None }; MAX_SLOT_COUNT as usize];
        slots[0] = Some(Instrument::from(Kind::Sine));
        slots[1] = Some(Instrument::from(Kind::Square));
        slots[2] = Some(Instrument::from(Kind::Sawtooth));
        slots[3] = Some(Instrument::from(Kind::Sample {
            name: "Piano".into(),
            signal: signal::stereo::Owned::from_path("assets/stereo.wav").unwrap(),
        }));
        Self {
            slots,
            selected_index: 0,
        }
    }
}

impl Instruments {
    pub fn get(&self, index: u8) -> Option<&Instrument> {
        self.slots.get(index as usize).and_then(Option::as_ref)
    }

    pub fn get_selected(&self) -> Option<&Instrument> {
        self.slots
            .get(self.selected_index as usize)
            .and_then(Option::as_ref)
    }

    pub fn selected_index(&self) -> u8 {
        self.selected_index
    }

    pub fn increment_selected(&mut self, increment: i32) {
        let mut selected_index = self.selected_index as i32;
        selected_index += increment;
        selected_index = selected_index.rem_euclid(const { MAX_SLOT_COUNT as i32 });
        self.selected_index = selected_index as u8;
    }
}
