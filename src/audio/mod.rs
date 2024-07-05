use joy_value_object::{mk_vo, mk_vo_consts};

pub mod dsp;
pub mod frame;
pub mod player;
pub mod signal;

mk_vo! {
    pub Volume: f32,
    default: 1.0,
    min: 0.0,
    max: 1.0,
}

mk_vo! {
    pub Pan: f32,
    default: 0.0,
    min: -1.0,
    max: 1.0,
}

mk_vo_consts! {
    Pan,
    LEFT => Pan::MIN_VALUE,
    RIGHT => Pan::MAX_VALUE,
}

impl Pan {
    pub fn left_volume(&self) -> Volume {
        Volume::new_unchecked(1.0 - self.value().clamp(0.0, 1.0))
    }

    pub fn right_volume(&self) -> Volume {
        Volume::new_unchecked(1.0 + self.value().clamp(-1.0, 0.0))
    }
}
