use joy_vector::Vector;

pub mod channel;
pub mod midi;
pub mod pattern;
pub mod song;

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
