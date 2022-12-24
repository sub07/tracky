use std::fmt::{Display, Formatter};

use anyhow::bail;

use crate::draw::Draw;
use crate::font_atlas::TextAlignment;
use crate::renderer::Renderer;

pub struct Pattern {
    lines: Vec<PatternLine>,
}

pub struct PatternLine {
    pub note: Note,
}

impl Default for PatternLine {
    fn default() -> Self {
        PatternLine {
            note: Note::Empty,
        }
    }
}

#[derive(Debug)]
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

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Note::A => "A",
            Note::B => "B",
            Note::C => "C",
            Note::D => "D",
            Note::E => "E",
            Note::F => "F",
            Note::G => "G",
            Note::CSharp => "C#",
            Note::DSharp => "D#",
            Note::FSharp => "F#",
            Note::GSharp => "A#",
            Note::ASharp => "B#",
            Note::Empty => "--"
        };
        write!(f, "{str}")
    }
}

impl Pattern {
    pub fn new(length: usize) -> Pattern {
        let mut lines = Vec::new();
        lines.resize_with(length, Default::default);
        Pattern {
            lines,
        }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn set(&mut self, index: usize, note: Note) -> anyhow::Result<()> {
        if index > self.lines.len() { bail!("Invalid index") }
        self.lines[index] = PatternLine { note };
        Ok(())
    }
}

impl Draw for Note {
    type DrawData = bool;

    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, selected: bool) {
        if selected {
            renderer.draw_rect(x, y, renderer.glyph_width() * 2, renderer.glyph_height(), (255, 255, 255));
        }
        renderer.draw_text(format!("{self}"), x, y, if selected { (0, 0, 0) } else { (255, 255, 255) }, TextAlignment::Left);
    }
}

impl Draw for PatternLine {
    type DrawData = (usize, usize);

    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, (index, cursor): (usize, usize)) {
        if index == cursor {
            renderer.draw_rect(x - (renderer.glyph_width() as i32) * 4, y, renderer.glyph_width() * 10, renderer.glyph_height(), (100, 100, 100));
        }
        renderer.draw_text(format!("{index}"), x + renderer.glyph_width() as i32, y, (255, 255, 255), TextAlignment::Right);
        self.note.draw(renderer, x + (renderer.glyph_width() * 2) as i32, y, index == cursor);
    }
}

impl Draw for Pattern {
    type DrawData = usize;

    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, cursor: usize) {
        let mut y = y;
        for (index, pattern_line) in self.lines.iter().enumerate() {
            pattern_line.draw(renderer, x, y, (index, cursor));
            y += renderer.glyph_height() as i32;
        }
    }
}