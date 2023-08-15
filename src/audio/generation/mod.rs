use std::f64::consts::PI;

use rust_utils_macro::New;

#[derive(New)]
pub struct SineWaveDescriptor {
    #[new_default]
    phase: f64,
    amp: f64,
    freq: f64,
    sample_rate: f64,
}

impl Iterator for SineWaveDescriptor {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let sample = (self.phase.sin() * self.amp) as f32;
        self.phase += 2.0 * PI * self.freq / self.sample_rate;
        Some((sample, sample))
    }
}

pub struct SquareWaveDescriptor {
    phase: f64,
    amp: f64,
    sample_rate: f64,
    freq_period: f64,
    half_freq_period: f64,
}

impl SquareWaveDescriptor {
    pub fn new(freq: f64, amp: f64, sample_rate: f64) -> Self {
        Self {
            phase: 0.0,
            amp,
            sample_rate,
            freq_period: 1.0 / freq,
            half_freq_period: 1.0 / freq / 2.0,
        }
    }
}

impl Iterator for SquareWaveDescriptor {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let sample = if self.phase < self.half_freq_period {
            1.0
        } else {
            -1.0
        } * self.amp;
        let sample = sample as f32;

        self.phase += 1.0 / self.sample_rate;
        if self.phase > self.freq_period {
            self.phase -= self.freq_period;
        }

        Some((sample, sample))
    }
}

pub struct SawWaveDescriptor {
    phase: f64,
    freq_period: f64,
    amp: f64,
    sample_rate: f64,
}

impl SawWaveDescriptor {
    pub fn new(freq: f64, amp: f64, sample_rate: f64) -> Self {
        Self {
            phase: 0.0,
            freq_period: 1.0 / freq,
            amp,
            sample_rate,
        }
    }
}

impl Iterator for SawWaveDescriptor {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let normalized_position = self.phase / self.freq_period;
        let sample = -1.0 + (1.0 - -1.0) * normalized_position * self.amp;
        let sample = sample as f32;
        self.phase += 1.0 / self.sample_rate;
        if self.phase > self.freq_period {
            self.phase -= self.freq_period;
        }

        Some((sample, sample))
    }
}
