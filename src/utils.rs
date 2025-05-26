use joy_vector::Vector;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn vector(self) -> Vector<i32, 2> {
        match self {
            Direction::Up => Vector::<_, 2>::new(0, -1),
            Direction::Down => Vector::<_, 2>::new(0, 1),
            Direction::Left => Vector::<_, 2>::new(-1, 0),
            Direction::Right => Vector::<_, 2>::new(1, 0),
        }
    }

    pub const fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    pub const fn is_vertical(self) -> bool {
        !self.is_horizontal()
    }
}

// Took the code from near_o11y crate: https://github.com/near/nearcore
pub mod invariants {
    ///
    /// If assert fails, panic on debug, and log error on release
    ///
    #[macro_export]
    macro_rules! assert_log {
        ($cond:expr) => {
            $crate::assert_log!($cond, "assertion failed: {}", stringify!($cond))
        };

        ($cond:expr, $fmt:literal $($arg:tt)*) => {
            if cfg!(debug_assertions) {
                assert!($cond, $fmt $($arg)*);
            } else {
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !$cond {
                    log::error!($fmt $($arg)*);
                }
            }
        };
    }

    #[macro_export]
    macro_rules! assert_log_bail {
        ($cond:expr) => {
            $crate::assert_log!($cond, "assertion failed: {}", stringify!($cond))
        };

        ($cond:expr, $fmt:literal $($arg:tt)*) => {
            if cfg!(debug_assertions) {
                assert!($cond, $fmt $($arg)*);
            } else {
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !$cond {
                    log::error!($fmt $($arg)*);
                    return;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! assert_log_fail {
        ($fmt:literal $($arg:tt)*) => {
            $crate::assert_log!(false, $fmt $($arg)*)
        };
    }
}

pub mod math {
    use std::f32::consts::PI;

    pub const TWO_PI: f32 = 2.0 * PI;
}

pub mod ratatui_buffer_safety {
    use std::panic::panic_any;

    use easy_ext::ext;
    use log::error;
    use ratatui::{
        buffer::{self, Buffer},
        layout::Position,
        style::Style,
    };

    #[ext(BufferExt)]
    pub impl Buffer {
        fn set_cell<P: Into<Position>>(&mut self, position: P, style: Style) {
            let position: Position = position.into();
            if let Some(cell) = self.cell_mut((position.x, position.y)) {
                cell.set_style(style);
            } else {
                let error_message = format!("out of bound access on buffer: tried to get cell ({}, {}) on a buffer with size ({}, {})", position.x, position.y, self.area().width, self.area().height);
                if cfg!(debug_assertions) {
                    panic_any(error_message);
                } else {
                    error!("{error_message}");
                }
            }
        }
    }
}
