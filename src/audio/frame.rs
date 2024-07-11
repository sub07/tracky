pub trait FrameExt: Clone + Copy {
    type Sample: Clone + Copy;

    const FRAME_SIZE: usize;
}

pub type StereoFrame = (f32, f32);

impl FrameExt for StereoFrame {
    type Sample = f32;
    const FRAME_SIZE: usize = 2;
}

pub type MonoFrame = f32;

impl FrameExt for MonoFrame {
    type Sample = f32;
    const FRAME_SIZE: usize = 1;
}
