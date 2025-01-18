use std::iter;

use itertools::Itertools;
use joy_macro::EnumIter;

use crate::audio::{
    frame::{Frame, StereoFrame},
    signal, synthesis, Pan, Volume,
};

#[derive(Clone)]
pub enum Kind {
    Sine,
    Square,
    Sawtooth,
    Sample(signal::stereo::Owned),
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
    ) -> Option<StereoFrame> {
        match &self.source {
            Kind::Sine => Some(synthesis::sine_wave(freq, volume, pan, phase, frame_rate)),
            Kind::Square => Some(synthesis::square_wave(freq, volume, pan, phase, frame_rate)),
            Kind::Sawtooth => Some(synthesis::sawtooth_wave(
                freq, volume, pan, phase, frame_rate,
            )),
            Kind::Sample(_) => todo!(),
        }
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
        for (output, generated) in signal.iter_mut().zip(
            iter::repeat_with(|| self.next_frame(freq, volume, pan, phase, frame_rate))
                .while_some(),
        ) {
            *output = generated;
        }
    }
}
