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
}

// Took the code from near_o11y crate: https://github.com/near/nearcore
pub mod invariants {
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
    macro_rules! assert_log_fail {
        ($fmt:literal $($arg:tt)*) => {
            $crate::assert_log!(false, $fmt $($arg)*)
        };
    }
}
