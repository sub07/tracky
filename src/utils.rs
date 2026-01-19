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
            Self::Up => Vector::<_, 2>::new(0, -1),
            Self::Down => Vector::<_, 2>::new(0, 1),
            Self::Left => Vector::<_, 2>::new(-1, 0),
            Self::Right => Vector::<_, 2>::new(1, 0),
        }
    }

    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
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

    #[easy_ext::ext(ApproxEq)]
    pub impl f32 {
        fn approx_eq(self, other: f32, epsilon: f32) -> bool {
            Self::abs(self - other) < epsilon
        }
    }
}
