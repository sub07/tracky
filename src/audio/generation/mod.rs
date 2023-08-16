use std::f32::consts::PI;

use rust_utils_macro::New;

use super::Samples;

#[derive(New)]
pub struct SineWaveDescriptor {
    #[new_default]
    phase: f32,
    amp: f32,
}

impl Samples for SineWaveDescriptor {
    fn next(&mut self, freq: f32, sample_rate: f32) -> Option<(f32, f32)> {
        let sample = self.phase.sin() * self.amp;
        self.phase += 2.0 * PI * freq / sample_rate;
        Some((sample, sample))
    }
}

#[derive(New)]
pub struct SquareWaveDescriptor {
    #[new_default]
    phase: f32,
    amp: f32,
}

impl Samples for SquareWaveDescriptor {
    fn next(&mut self, freq: f32, sample_rate: f32) -> Option<(f32, f32)> {
        let half_freq_period = 1.0 / freq / 2.0;
        let freq_period = 1.0 / freq;

        let sample = if self.phase < half_freq_period {
            1.0
        } else {
            -1.0
        } * self.amp;
        let sample = sample as f32;

        self.phase += 1.0 / sample_rate;
        if self.phase > freq_period {
            self.phase -= freq_period;
        }

        Some((sample, sample))
    }
}

#[derive(New)]
pub struct SawWaveDescriptor {
    #[new_default]
    phase: f32,
    amp: f32,
}

impl Samples for SawWaveDescriptor {
    fn next(&mut self, freq: f32, sample_rate: f32) -> Option<(f32, f32)> {
        let freq_period = 1.0 / freq;
        let normalized_position = self.phase / freq_period;
        let sample = -1.0 + (1.0 - -1.0) * normalized_position * self.amp;
        let sample = sample as f32;
        self.phase += 1.0 / sample_rate;
        if self.phase > freq_period {
            self.phase -= freq_period;
        }

        Some((sample, sample))
    }
}
