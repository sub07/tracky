use std::{
    cell::{LazyCell, OnceCell},
    iter,
    sync::LazyLock,
    time::Duration,
};

use itertools::Itertools;
use joy_macro::EnumIter;
use joy_vector::{vector, Vector};

use crate::{
    audio::{
        frame::{Frame, StereoFrame},
        signal, synthesis, Pan, Volume,
    },
    model::{
        midi::note_to_freq,
        pattern::{NoteName, OctaveValue},
    },
};

use super::midi::C5_FREQ;

#[derive(Clone)]
pub enum Kind {
    Sine,
    Square,
    Sawtooth,
    Sample(signal::stereo::Owned),
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

#[derive(Clone)]
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
