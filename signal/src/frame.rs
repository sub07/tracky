use joy_vector::Vector;

pub type Frame<const SIZE: usize> = Vector<f32, SIZE>;
pub type StereoFrame = Frame<2>;