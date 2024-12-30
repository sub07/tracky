use std::f32::consts::PI;

use joy_vector::vector;

use super::{
    frame::{StereoFrame, YieldFrame},
    Pan, Volume,
};

pub struct SineWave;

impl YieldFrame for SineWave {
    fn next(
        &mut self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<StereoFrame> {
        let sample = phase.sin();
        *phase += 2.0 * PI * freq / sample_rate;

        let left_volume = pan.left_volume() * volume;
        let right_volume = pan.right_volume() * volume;

        Some(vector!(
            sample * left_volume.value(),
            sample * right_volume.value(),
        ))
    }
}

pub struct SquareWave;

impl YieldFrame for SquareWave {
    fn next(
        &mut self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<StereoFrame> {
        let freq_period = 1.0 / freq;
        let half_freq_period = freq_period / 2.0;

        let sample = if *phase < half_freq_period { 1.0 } else { -1.0 };

        *phase += 1.0 / sample_rate;
        if *phase > freq_period {
            *phase -= freq_period;
        }

        let left_volume = volume * pan.left_volume();
        let right_volume = volume * pan.right_volume();

        Some(vector!(
            sample * left_volume.value(),
            sample * right_volume.value(),
        ))
    }
}

pub struct SawWave;

impl YieldFrame for SawWave {
    fn next(
        &mut self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<StereoFrame> {
        let freq_period = 1.0 / freq;

        let sample = *phase / freq_period;

        *phase += 1.0 / sample_rate;
        if *phase > freq_period {
            *phase -= freq_period;
        }

        let left_volume = volume * pan.left_volume();
        let right_volume = volume * pan.right_volume();

        Some(vector!(
            sample * left_volume.value(),
            sample * right_volume.value(),
        ))
    }
}
