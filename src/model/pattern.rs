use std::slice::Iter;

use crate::model::pattern_line::PatternLine;

pub struct Pattern {
    lines: Vec<PatternLine>,
}

impl Pattern {
    pub fn new(length: usize) -> Pattern {
        let mut lines = Vec::new();
        lines.resize_with(length, Default::default);
        Pattern {
            lines,
        }
    }

    pub fn iter(&self) -> Iter<PatternLine> {
        self.lines.iter()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}
