use std::iter;

use itertools::Itertools;
use joy_vector::Vector;

use super::{signal::StereoSigMut, Pan, Volume};

pub type Frame<const SIZE: usize> = Vector<f32, SIZE>;
pub type StereoFrame = Frame<2>;

pub trait YieldFrame {
    fn next(
        &mut self,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<StereoFrame>;

    fn collect_in(
        &mut self,
        mut signal: StereoSigMut,
        freq: f32,
        volume: Volume,
        pan: Pan,
        phase: &mut f32,
    ) {
        signal.fill(Frame::default());
        let frame_rate = signal.frame_rate;
        for (output, generated) in signal
            .iter_mut()
            .zip(iter::repeat_with(|| self.next(freq, volume, pan, phase, frame_rate)).while_some())
        {
            *output = generated;
        }
    }
}
