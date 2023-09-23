use std::f32::consts::PI;

use super::Samples;

pub struct SineWaveDescriptor;

impl Samples for SineWaveDescriptor {
    fn next(
        &mut self,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<(f32, f32)> {
        let s = phase.sin() * amp;
        *phase += 2.0 * PI * freq / sample_rate;
        Some((s, s))
    }
}

pub struct SquareWaveDescriptor;

impl Samples for SquareWaveDescriptor {
    fn next(
        &mut self,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<(f32, f32)> {
        let half_freq_period = 1.0 / freq / 2.0;
        let freq_period = 1.0 / freq;

        let sample = if *phase < half_freq_period { 1.0 } else { -1.0 } * amp;
        let s = sample as f32;

        *phase += 1.0 / sample_rate;
        if *phase > freq_period {
            *phase -= freq_period;
        }

        Some((s, s))
    }
}
pub struct SawWaveDescriptor;

impl Samples for SawWaveDescriptor {
    fn next(
        &mut self,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<(f32, f32)> {
        let freq_period = 1.0 / freq;
        let normalized_position = *phase / freq_period;
        let sample = -1.0 + (1.0 - -1.0) * normalized_position * amp;
        let s = sample as f32;
        *phase += 1.0 / sample_rate;
        if *phase > freq_period {
            *phase -= freq_period;
        }

        Some((s, s))
    }
}
