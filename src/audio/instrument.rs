use crate::audio::FrameIterator;

pub struct Instrument {
    pub frame_iter: Box<dyn FrameIterator>,
    pub index: u8,
    pub phase: f32,
}