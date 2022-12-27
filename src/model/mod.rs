pub mod pattern_line;
pub mod pattern;
pub mod patterns;
pub mod field;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn vec(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

pub enum PatternLineElement {
    Note,
    Velocity,
}

impl PatternLineElement {
    pub const fn line_len() -> usize {
        let mut sum = 0;
        let mut i = 0;
        while i < PATTERN_LINE_ELEMENT_ARRAY.len() {
            sum += PATTERN_LINE_ELEMENT_ARRAY[i].len();
            i += 1;
        }
        sum
    }

    pub const fn len(&self) -> usize {
        match self {
            PatternLineElement::Note => 3,
            PatternLineElement::Velocity => 2,
        }
    }
}

const PATTERN_LINE_ELEMENT_ARRAY: [PatternLineElement; 2] = [PatternLineElement::Note, PatternLineElement::Velocity];
pub const PATTERN_LINE_LEN: usize = PatternLineElement::line_len();

pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    CSharp,
    DSharp,
    FSharp,
    GSharp,
    ASharp,
    Empty,
}
