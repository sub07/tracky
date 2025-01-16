pub mod field;

use std::time::Duration;

use itertools::Itertools;
use joy_vector::Vector;
use log::{error, warn};

use crate::{
    audio::{
        frame::Frame,
        signal::{self, Owned},
    },
    model::{
        self,
        channel::Channel,
        pattern::{Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLineDescriptor},
        playback::song::SongPlayback,
    },
    utils::Direction,
};

///
/// Event handling methods
///
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
            model::Event::PerformStepPlayback => self.perform_step_playback(),
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
        let current_field = self.patterns.current_field;
        let line = self.patterns.current_line_mut();
        let field = match PatternLineDescriptor::field_by_cursor(current_field) {
            PatternLineDescriptor::Velocity => &mut line.velocity,
            PatternLineDescriptor::Instrument => &mut line.instrument,
            _ => unreachable!(),
        };
        field.set_by_index(current_field, digit);
    }

    fn start_song_playback(&mut self, frame_rate: f32) {
        let mut channels = vec![Channel::new(); self.patterns.channel_count as usize];

        // Init first line
        for (line, channel) in self.patterns.current_pattern_row(0).zip(&mut channels) {
            channel.setup_line(line);
        }

        self.playback = Some(SongPlayback {
            channels,
            step_signal: signal::stereo::Owned::new(frame_rate),
            line_signal: signal::stereo::Owned::new(frame_rate),
            current_line: 0,
            current_line_duration: Duration::ZERO,
            line_duration: Duration::from_secs_f32(1.0 / self.line_per_second),
            last_step_computed_sample_count: 0,
        });
    }

    fn stop_song_playback(&mut self) {
        self.playback = None;
    }

    fn update_playback_sample_count(&mut self, new_sample_count: usize) {
        let Some(playback) = self.playback.as_mut() else {
            error!("Attempting to update sample_count with no active playback");
            return;
        };
        warn!(
            "Heap allocations triggered: playback sample count changed from {} to {}",
            playback.step_signal.as_ref().sample_count(),
            new_sample_count
        );
        playback.line_signal =
            Owned::from_sample_count(new_sample_count, playback.line_signal.frame_rate);
        playback.step_signal = playback.line_signal.clone();
    }

    fn perform_step_playback(&mut self) {
        let Some(playback) = self.playback.as_mut() else {
            error!("Attempting to perform playback without any active playback");
            return;
        };

        playback.last_step_computed_sample_count = 0;

        if playback.current_line as i32 >= self.patterns.channel_len {
            return;
        }

        playback.step_signal.fill(Frame::default());

        let step_duration = playback.step_signal.as_ref().duration();
        let mut sub_step_start_duration = Duration::ZERO;

        while sub_step_start_duration < step_duration {
            let sub_step_duration = (playback.line_duration - playback.current_line_duration)
                .min(step_duration - sub_step_start_duration);

            let sub_step_end_duration = sub_step_start_duration + sub_step_duration;

            for channel in playback.channels.iter_mut() {
                channel.collect_signal(
                    playback
                        .line_signal
                        .sub_signal_mut(sub_step_start_duration, sub_step_end_duration)
                        .unwrap(),
                );
                for (mut out, input) in playback
                    .step_signal
                    .sub_signal_mut(sub_step_start_duration, sub_step_end_duration)
                    .unwrap()
                    .iter_mut()
                    .zip_eq(
                        playback
                            .line_signal
                            .sub_signal(sub_step_start_duration, sub_step_end_duration)
                            .unwrap()
                            .iter(),
                    )
                {
                    out += input;
                }
            }

            sub_step_start_duration = sub_step_end_duration;

            playback.current_line_duration += sub_step_duration;
            if playback.current_line_duration >= playback.line_duration {
                playback.current_line += 1;
                if playback.current_line as i32 >= self.patterns.channel_len {
                    break;
                }

                playback.current_line_duration -= playback.line_duration;
                for (line, channel) in self
                    .patterns
                    .current_pattern_row(playback.current_line)
                    .zip(&mut playback.channels)
                {
                    channel.setup_line(line);
                }
            }
        }

        playback.last_step_computed_sample_count = (sub_step_start_duration.as_secs_f32()
            * playback.step_signal.frame_rate
            * 2.0) as usize;
    }
}
