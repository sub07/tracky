use std::{iter, ops, time::Duration};

use itertools::Itertools;
use joy_vector::Vector;

use super::{signal, Pan, Volume};

pub type Frame<const SIZE: usize> = Vector<f32, SIZE>;
pub type StereoFrame = Frame<2>;

impl<const SIZE: usize> ops::Mul<Volume> for Frame<SIZE> {
    type Output = Frame<SIZE>;

    fn mul(self, rhs: Volume) -> Self::Output {
        self * rhs.value()
    }
}
