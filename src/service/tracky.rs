use std::time::Instant;

use iced::{widget::scrollable, Command, Event};
use iter_tools::Itertools;

use crate::{
    audio::{audio_channel::AudioChannel, signal::StereoSignal},
    keybinding,
    model::{
        field::{
            value_object::{HexDigit, OctaveValue},
            NoteName,
        },
        pattern::PatternLineDescriptor,
    },
    PlayingState, Tracky,
};

impl Tracky {
    pub fn set_note_name(&mut self, note: NoteName) {
        let current_line = self.patterns.current_line_mut();

        current_line.note.set_note_name(note, self.default_octave);

        if current_line.instrument.value().is_none() {
            current_line.instrument.set_u8(self.selected_instrument);
        }
    }

    pub fn set_velocity(&mut self, digit: HexDigit) {
        match self.patterns.local_column_index() {
            3 => self
                .patterns
                .current_line_mut()
                .velocity
                .set_first_digit(digit),
            4 => self
                .patterns
                .current_line_mut()
                .velocity
                .set_second_digit(digit),
            _ => panic!("Should not happen"),
        };
    }

    pub fn set_instrument(&mut self, digit: HexDigit) {
        match self.patterns.local_column_index() {
            5 => self
                .patterns
                .current_line_mut()
                .instrument
                .set_first_digit(digit),
            6 => self
                .patterns
                .current_line_mut()
                .instrument
                .set_second_digit(digit),
            _ => panic!("Should not happen"),
        };
    }

    pub fn set_hex(&mut self, hex_value: HexDigit) {
        match self.patterns.local_column_index() {
            3 | 4 => self.set_velocity(hex_value),
            5 | 6 => self.set_instrument(hex_value),
            _ => {}
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        self.patterns.current_line_mut().note.set_octave(octave);
    }

    pub fn clear(&mut self) {
        match self.patterns.local_column_index() {
            0 | 2 => {
                self.patterns.current_line_mut().note.clear();
                self.patterns.current_line_mut().velocity.clear();
                self.patterns.current_line_mut().instrument.clear();
            }
            3 | 4 => self.patterns.current_line_mut().velocity.clear(),
            5 | 6 => self.patterns.current_line_mut().instrument.clear(),
            _ => {}
        }
    }

    pub fn move_cursor(
        &mut self,
        x: i32,
        y: i32,
    ) -> Command<<Tracky as iced::Application>::Message> {
        self.patterns.cursor_x += x;
        self.patterns.cursor_y += y;

        let (pattern_len, nb_column) = {
            let current_pattern = self.patterns.current_pattern();
            (current_pattern.len, current_pattern.nb_column)
        };

        if self.patterns.cursor_x % PatternLineDescriptor::LINE_LEN == 1 {
            self.patterns.cursor_x += x;
        }

        self.patterns.cursor_x = i32::rem_euclid(
            self.patterns.cursor_x,
            PatternLineDescriptor::LINE_LEN * nb_column as i32,
        );
        self.patterns.cursor_y = i32::rem_euclid(self.patterns.cursor_y, pattern_len as i32);

        let cursor_x_column_index = self.patterns.cursor_x / PatternLineDescriptor::LINE_LEN;

        scrollable::snap_to(
            self.pattern_scroll_id.clone(),
            scrollable::RelativeOffset {
                x: cursor_x_column_index as f32 / (nb_column - 1) as f32,
                y: self.patterns.cursor_y as f32 / (pattern_len - 1) as f32,
            },
        )
    }

    pub fn convert_event_to_action(&self, event: Event) -> Option<keybinding::Action> {
        let input_context = self.patterns.input_type();
        match event {
            Event::Keyboard(kb_event) => match kb_event {
                iced::keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                } => self.keybindings.action(modifiers, key_code, input_context),
                iced::keyboard::Event::KeyReleased {
                    key_code: _,
                    modifiers: _,
                } => None,
                iced::keyboard::Event::CharacterReceived(_) => None,
                iced::keyboard::Event::ModifiersChanged(_) => None,
            },
            Event::Mouse(_) => None,
            Event::Window(_) => None,
            Event::Touch(_) => None,
        }
    }

    pub fn line_per_second(&self) -> f32 {
        self.line_per_minute / 60.0
    }

    pub fn play_line(&mut self, now: Instant) {
        let line_per_second = self.line_per_second();
        if let PlayingState::Playing {
            player,
            current_line,
            last_time,
            should_handle_next_line,
        } = &mut self.playing_state
        {
            let dt = now - *last_time;
            *last_time = now;
            *current_line += line_per_second * dt.as_secs_f32();
            let old_cursor_y = self.patterns.cursor_y;
            self.patterns.cursor_y = *current_line as i32;
            *should_handle_next_line = old_cursor_y != self.patterns.cursor_y;

            let mut channels = (0..self.patterns.nb_channel)
                .map(|_| AudioChannel::new(player.sample_rate, line_per_second))
                .collect_vec();
            for (line, channel) in self
                .patterns
                .current_pattern()
                .columns()
                .map(|c| &c.lines[self.patterns.cursor_y as usize])
                .zip(&mut channels)
            {
                channel.process_line(line);
            }

            let mixer = channels.iter().fold(
                StereoSignal::new(
                    AudioChannel::compute_buffer_duration(line_per_second),
                    player.sample_rate,
                ),
                |mut mixer, channel| {
                    mixer += channel.buffer();
                    mixer
                },
            );

            player.queue(&mixer).unwrap();
        }
    }
}
