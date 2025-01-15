pub mod field;

use std::time::Duration;

use joy_vector::Vector;
use log::{error, warn};

use crate::{
    audio::mixer::Mixer,
    model::{
        self,
        channel::Channel,
        pattern::{Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLineDescriptor},
        playback::song::SongPlayback,
    },
    utils::Direction,
};

impl model::State {
    pub fn handle_event(&mut self, event: model::Event) {
        match event {
            model::Event::MutateGlobalOctave { increment } => self.mutate_global_octave(increment),
            model::Event::SetNoteField {
                note,
                octave_modifier,
            } => self.set_note_field(note, octave_modifier),
            model::Event::MoveCursor(direction) => self.move_cursor(direction),
            model::Event::SetNoteFieldToCut => self.set_note_field_to_cut(),
            model::Event::ClearField => self.clear_field(),
            model::Event::SetOctaveField(octave) => self.set_octave_field(octave),
            model::Event::SetHexField(digit) => self.set_hex_field(digit),
            model::Event::NewPattern => todo!(),
            model::Event::NextPattern => todo!(),
            model::Event::PreviousPattern => todo!(),
            model::Event::StartSongPlayback { frame_rate } => self.start_song_playback(frame_rate),
            model::Event::StopSongPlayback => self.stop_song_playback(),
            model::Event::UpdatePlaybackSampleCount(new_sample_count) => {
                self.update_playback_sample_count(new_sample_count)
            }
        }
    }

    fn mutate_global_octave(&mut self, increment: i32) {
        self.global_octave = self.global_octave + increment;
    }

    fn set_note_field(&mut self, note: NoteName, octave_modifier: i32) {
        let line = self.patterns.current_line_mut();
        line.note
            .set_note_name(note, self.global_octave + octave_modifier);
        if line.instrument.value().is_none() {
            line.instrument.set((HexDigit::HEX_0, HexDigit::HEX_0));
        }
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction.vector() {
            // Vertical
            Vector([0, d]) => {
                self.patterns.current_row += d;
                self.patterns.current_row = self
                    .patterns
                    .current_row
                    .rem_euclid(self.patterns.channel_len);
            }
            // Horizontal
            Vector([d, 0]) => {
                self.patterns.current_field += d;

                self.patterns.current_channel += self
                    .patterns
                    .current_field
                    .div_euclid(PatternLineDescriptor::LINE_LEN);

                self.patterns.current_field = self
                    .patterns
                    .current_field
                    .rem_euclid(PatternLineDescriptor::LINE_LEN);
                self.patterns.current_channel = self
                    .patterns
                    .current_channel
                    .rem_euclid(self.patterns.channel_count);
            }
            _ => unreachable!(),
        }
    }

    fn set_note_field_to_cut(&mut self) {
        self.patterns
            .current_line_mut()
            .note
            .set(NoteFieldValue::Cut);
    }

    fn clear_field(&mut self) {
        let field_cursor = self.patterns.current_field;
        let line = self.patterns.current_line_mut();
        match PatternLineDescriptor::field_by_cursor(field_cursor) {
            PatternLineDescriptor::Note => {
                line.note.clear();
                line.velocity.clear();
                line.instrument.clear();
            }
            PatternLineDescriptor::Velocity => line.velocity.clear(),
            PatternLineDescriptor::Instrument => line.instrument.clear(),
        }
    }

    fn set_octave_field(&mut self, octave: OctaveValue) {
        self.patterns.current_line_mut().note.set_octave(octave);
    }

    fn set_hex_field(&mut self, digit: HexDigit) {
        match PatternLineDescriptor::field_by_cursor(self.patterns.current_field) {
            PatternLineDescriptor::Velocity => set_double_hex_field(
                self.patterns.current_field,
                &mut self.patterns.current_line_mut().velocity,
                digit,
            ),
            PatternLineDescriptor::Instrument => set_double_hex_field(
                self.patterns.current_field,
                &mut self.patterns.current_line_mut().instrument,
                digit,
            ),
            _ => unreachable!(),
        }
    }

    fn start_song_playback(&mut self, frame_rate: f32) {
        let line_duration = Duration::from_secs_f32(1.0 / self.line_per_second);
        let master = Mixer::from_sample_buffer_size(0, frame_rate);
        let channels = vec![Channel::new(); self.patterns.channel_count as usize];

        self.playback = Some(SongPlayback {
            channels,
            master,
            current_line: 0,
            line_duration,
            time_since_last_line: Duration::ZERO,
        });
    }

    fn stop_song_playback(&mut self) {
        self.playback = None;
    }

    fn update_playback_sample_count(&mut self, new_sample_count: usize) {
        if let Some(playback) = self.playback.as_mut() {
            warn!(
                "Allocation: master mixer sample count changed from {} to {}",
                playback.master.sample_count(),
                new_sample_count
            );
            playback.master =
                Mixer::from_sample_buffer_size(new_sample_count, playback.master.signal.frame_rate);
        } else {
            error!("Attempting to update playback params, but no playback is active");
        }
    }
}

fn set_double_hex_field(
    field_cursor: i32,
    field: &mut Field<(HexDigit, HexDigit)>,
    digit: HexDigit,
) {
    match PatternLineDescriptor::local_field_cursor(field_cursor) {
        0 => field.set_first_digit(digit),
        1 => field.set_second_digit(digit),
        _ => unreachable!(),
    }
}
