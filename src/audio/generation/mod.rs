use std::{f32::consts::PI, time::Duration};

use super::Samples;

const VELOCITY_TRANSITION_DURATION: Duration = Duration::from_millis(20);

#[derive(Default)]
enum InterpolationState<T> {
    #[default]
    Stable,
    Interpolating {
        goal: T,
        current: T,
        incr: T,
    },
}

#[derive(Default)]
struct ParameterInterpolator<T> {
    state: InterpolationState<T>,
    last_value: Option<T>,
}

impl<T> ParameterInterpolator<T>
where
    T: Clone
        + Copy
        + From<f32>
        + std::cmp::PartialEq
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Add<Output = T>
        + std::cmp::PartialOrd
{
    pub fn process_new_value(&mut self, new_value: T, sample_rate: f32) -> T {
        let last_value = if let Some(v) = self.last_value {
            v
        } else {
            new_value
        };

        let value = match self.state {
            InterpolationState::Stable => {
                if last_value != new_value {
                    let goal = new_value;
                    let current = last_value;
                    let nb_frames = VELOCITY_TRANSITION_DURATION.as_secs_f32() * sample_rate;
                    let incr = (goal - current) / nb_frames.into();
                    self.state = InterpolationState::Interpolating {
                        goal,
                        current,
                        incr,
                    };
                    self.process_new_value(new_value, sample_rate)
                } else {
                    new_value
                }
            }
            InterpolationState::Interpolating {
                goal,
                ref mut current,
                incr,
            } => {
                if (incr > 0.0.into() && *current > goal) || (incr < 0.0.into() && *current < goal) {
                    // Interpolation done
                    self.state = InterpolationState::Stable;
                    self.process_new_value(new_value, sample_rate)
                } else {
                    *current = *current + incr;
                    *current
                }
            }
        };
        self.last_value = Some(new_value);
        value
    }
}

pub struct SampleParametersInterpolator<S: Samples> {
    samples: S,
    amp_interpolator: ParameterInterpolator<f32>,
}

impl<S: Samples> SampleParametersInterpolator<S> {
    pub fn new(samples: S) -> SampleParametersInterpolator<S> {
        SampleParametersInterpolator {
            samples,
            amp_interpolator: Default::default(),
        }
    }
}

impl<S: Samples> Samples for SampleParametersInterpolator<S> {
    fn next(
        &mut self,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<(f32, f32)> {
        let current_amp = self.amp_interpolator.process_new_value(amp, sample_rate);
        self.samples.next(freq, current_amp, phase, sample_rate)
    }
}

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
