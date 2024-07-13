use std::time::Duration;

use itertools::Itertools;
use joy_vector::Vector;

use super::{
    signal::{Signal, StereoSignal},
    Pan, Volume,
};

pub type Frame<const SIZE: usize> = Vector<f32, SIZE>;
pub type StereoFrame = Frame<2>;

pub trait CollectFrame {
    fn next(
        &mut self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<StereoFrame>;

    fn collect_for_duration(
        &mut self,
        duration: Duration,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> StereoSignal {
        let frame_count = sample_rate * duration.as_secs_f32();
        let frames = (0..frame_count as usize)
            .map_while(|_| self.next(freq, volume, pan, phase, sample_rate))
            .collect_vec();
        Signal::from_frames(frames, sample_rate)
    }
}
