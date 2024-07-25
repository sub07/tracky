use std::fmt::Debug;

use anyhow::anyhow;
use derivative::Derivative;

use joy_macro::EnumIter;
use joy_value_object::{mk_vo, mk_vo_consts};

use crate::keybindings;

mk_vo! {
    pub HexDigit: u8,
    default: 0,
    min: 0,
    max: 0xF,
}

mk_vo! {
    pub OctaveValue: i32,
    default: 5,
    min: 0,
    max: 9,
}

mk_vo_consts! {
    HexDigit,
    HEX_0 => 0x0,
    HEX_1 => 0x1,
    HEX_2 => 0x2,
    HEX_3 => 0x3,
    HEX_4 => 0x4,
    HEX_5 => 0x5,
    HEX_6 => 0x6,
    HEX_7 => 0x7,
    HEX_8 => 0x8,
    HEX_9 => 0x9,
    HEX_A => 0xA,
    HEX_B => 0xB,
    HEX_C => 0xC,
    HEX_D => 0xD,
    HEX_E => 0xE,
    HEX_F => 0xF,
}

mk_vo_consts! {
    OctaveValue,
    OCTAVE_0 => 0x0,
    OCTAVE_1 => 0x1,
    OCTAVE_2 => 0x2,
    OCTAVE_3 => 0x3,
    OCTAVE_4 => 0x4,
    OCTAVE_5 => 0x5,
    OCTAVE_6 => 0x6,
    OCTAVE_7 => 0x7,
    OCTAVE_8 => 0x8,
    OCTAVE_9 => 0x9,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<T>(Option<T>);

impl<T> Default for Field<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Field<T> {
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }

    pub fn empty() -> Self {
        Self(None)
    }

    pub fn set(&mut self, value: T) {
        self.0 = Some(value)
    }

    pub fn clear(&mut self) {
        self.0 = None
    }

    pub fn value(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter)]
pub enum NoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteFieldValue {
    Note(NoteName, OctaveValue),
    Cut,
}

macro_rules! declare_field {
    ($($snake_case:ident $pascal_case:ident $size:literal $ty:ty),* $(,)?) => {
        #[derive(Default, Debug, Clone, PartialEq)]
        pub struct PatternLine {
            $(
                pub $snake_case: Field<$ty>,
            )*
        }

        #[derive(joy_macro::EnumIter, PartialEq, Debug, Clone, Copy)]
        pub enum PatternLineDescriptor {
            $(
                $pascal_case,
            )*
        }

        impl PatternLineDescriptor {
            pub const LINE_LEN: i32 = $($size+)* 0;

            pub const fn field_len(self) -> usize {
                match self {
                    $(
                        Self::$pascal_case => $size,
                    )*
                }
            }
        }
    };
}

impl PatternLineDescriptor {
    pub const INDEX_BOUNDS: [(i32, i32); Self::COUNT] = Self::field_index_bounds_exclusive();

    pub const fn field_index_bounds_exclusive() -> [(i32, i32); Self::COUNT] {
        let mut sum = 0;
        let mut last_sum = 0;
        let mut i = 0;
        let mut indexes = [(0, 0); Self::COUNT];
        while i < Self::COUNT {
            sum += Self::VARIANTS[i].field_len() as i32;
            indexes[i] = (last_sum, sum);
            last_sum = sum;
            i += 1;
        }
        indexes
    }

    pub fn field_index_by_cursor(field_cursor: i32) -> usize {
        Self::INDEX_BOUNDS
            .iter()
            .take_while(|(_, e)| field_cursor >= *e)
            .count()
    }

    // field_cursor = 6
    // ... .. ..
    //         ^
    // local_cursor = 1
    pub fn local_field_cursor(field_cursor: i32) -> i32 {
        field_cursor - Self::INDEX_BOUNDS[Self::field_index_by_cursor(field_cursor)].0
    }

    pub fn field_by_cursor(field_cursor: i32) -> PatternLineDescriptor {
        Self::VARIANTS[Self::field_index_by_cursor(field_cursor)]
    }
}

declare_field! {
    note Note 3 NoteFieldValue,
    velocity Velocity 2 (HexDigit, HexDigit),
    instrument Instrument 2 (HexDigit, HexDigit),
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub lines: Vec<PatternLine>,
}

impl Pattern {
    pub fn new(channel_count: i32, channel_len: i32) -> Pattern {
        let len = (channel_count * channel_len) as usize;
        let mut lines = Vec::with_capacity(len);
        lines.resize_with(len, Default::default);
        Self { lines }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Patterns {
    #[derivative(Debug = "ignore")]
    patterns: Vec<Pattern>,
    pub channel_len: i32,
    pub channel_count: i32,
    pub pattern_count: i32,
    pub current_channel: i32,
    pub current_field: i32,
    pub current_row: i32,
    pub current_pattern: usize,
    pub default_octave: OctaveValue,
}

impl Default for Patterns {
    fn default() -> Self {
        let channel_count = 32;
        let channel_len = 128;
        let pattern_count = 1;
        Patterns::new(channel_count, channel_len, pattern_count)
    }
}

impl Patterns {
    pub fn new(channel_count: i32, channel_len: i32, pattern_count: i32) -> Patterns {
        let mut patterns = Vec::with_capacity(pattern_count as usize);
        patterns.resize_with(pattern_count as usize, || {
            Pattern::new(channel_count, channel_len)
        });

        debug_assert_eq!(
            channel_count * channel_len * pattern_count,
            patterns.len() as i32 * patterns.iter().map(|p| p.lines.len()).sum::<usize>() as i32
        );

        Patterns {
            patterns,
            channel_len,
            channel_count,
            pattern_count,
            current_channel: 0,
            current_field: 0,
            current_row: 0,
            current_pattern: 0,
            default_octave: Default::default(),
        }
    }

    fn current_pattern(&self) -> &Pattern {
        let pattern_index = self.current_pattern;
        self.patterns
            .get(pattern_index)
            .ok_or_else(|| anyhow!("Invalid state: {pattern_index}"))
            .unwrap()
    }

    fn current_pattern_mut(&mut self) -> &mut Pattern {
        let pattern_index = self.current_pattern;
        self.patterns
            .get_mut(pattern_index)
            .ok_or_else(|| anyhow!("Invalid state: {pattern_index}"))
            .unwrap()
    }

    pub fn current_pattern_channels(&self) -> impl Iterator<Item = &[PatternLine]> {
        self.current_pattern()
            .lines
            .as_slice()
            .chunks_exact(self.channel_len as usize)
    }

    pub fn current_input_context(&self) -> keybindings::InputContext {
        match (
            PatternLineDescriptor::field_by_cursor(self.current_field),
            PatternLineDescriptor::local_field_cursor(self.current_field),
        ) {
            (PatternLineDescriptor::Note, local_cursor) => match local_cursor {
                0 | 1 => keybindings::InputContext::Note,
                2 => keybindings::InputContext::Octave,
                _ => unreachable!(),
            },
            (PatternLineDescriptor::Velocity, _) => keybindings::InputContext::Hex,
            (PatternLineDescriptor::Instrument, _) => keybindings::InputContext::Hex,
        }
    }

    pub fn current_line_mut(&mut self) -> &mut PatternLine {
        let line_index = self.current_channel * self.channel_len + self.current_row;
        self.current_pattern_mut()
            .lines
            .get_mut(line_index as usize)
            .ok_or_else(|| anyhow!("Invalid state: {line_index}"))
            .unwrap()
    }
}
