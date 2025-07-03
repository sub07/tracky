pub mod field;

use std::time::Duration;

use joy_vector::Vector;

use crate::{
    assert_log,
    audio::{
        frame::Frame,
        signal::{self},
    },
    model::{
        self,
        channel::Channel,
        pattern::{
            u8_to_hex_digit_pair, HexDigit, NoteFieldValue, NoteName, OctaveValue,
            PatternLineDescriptor,
        },
    },
    utils::Direction,
};

///
/// Event handling methods
///
impl model::State {
    pub fn handle_command(&mut self, event: model::Command) {
        match event {
            model::Command::ChangeGlobalOctave { increment } => {
                self.change_global_octave(increment)
            }
            model::Command::SetNoteField {
                note,
                octave_modifier,
            } => self.set_note_field(note, octave_modifier),
            model::Command::MoveCursor(direction) => self.move_cursor(direction),
            model::Command::SetNoteCut => self.set_note_cut(),
            model::Command::ClearField => self.clear_field(),
            model::Command::SetOctaveField(octave) => self.set_octave_field(octave),
            model::Command::SetHexField(digit) => self.set_hex_field(digit),
            model::Command::CreateNewPattern => todo!(),
            model::Command::GoToNextPattern => todo!(),
            model::Command::GoToPreviousPattern => todo!(),
            model::Command::StartSongPlaybackFromBeginning => {
                self.start_song_playback_from_beginning()
            }
            model::Command::StopSongPlayback => self.stop_song_playback(),
            model::Command::UpdatePlaybackSampleCount(sample_count) => {
                self.audio_stream_sample_count_changed(sample_count)
            }
            model::Command::PerformPlaybacksStep => self.perform_playbacks_step(),
            model::Command::InitializeAudio { frame_rate } => self.initialize_audio(frame_rate),
            model::Command::ClearChannels => self.clear_channels(),
            model::Command::ChangeSelectedInstrument { increment } => {
                self.change_selected_instrument(increment)
            }
            model::Command::ChangeGlobalVolume { volume } => {
                self.global_volume = volume;
            }
        }
    }

    fn change_global_octave(&mut self, increment: i32) {
        // TODO: clarify implicit saturating add
        self.global_octave = self.global_octave + increment;
    }

    fn set_note_field(&mut self, note: NoteName, octave_modifier: i32) {
        let current_channel = self.patterns.current_channel as usize;
        let line = self.patterns.current_line_mut();
        line.note
            .set_note_name(note, self.global_octave + octave_modifier);
        line.instrument
            .set(u8_to_hex_digit_pair(self.instruments.selected_index()));
        self.channels[current_channel].setup_line(line);
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

    fn set_note_cut(&mut self) {
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

    fn start_song_playback_from_beginning(&mut self) {
        assert_log!(self.song_playback.is_some());
        let Some(song_playback) = self.song_playback.as_mut() else {
            return;
        };
        song_playback.current_line = 0;
        song_playback.is_playing = true;
        song_playback.current_line_duration = Duration::ZERO;

        if self.follow_playing {
            self.patterns.current_row = 0;
        }

        assert_log!(self.patterns.channel_count as usize == self.channels.len());

        // Init first line
        for (line, channel) in self
            .patterns
            .current_pattern_row(0)
            .zip(self.channels.iter_mut())
        {
            channel.setup_line(line);
        }
    }

    fn stop_song_playback(&mut self) {
        // TODO find a better way to panic on debug and log on release
        assert_log!(self.song_playback.is_some());
        let Some(song_playback) = self.song_playback.as_mut() else {
            return;
        };
        song_playback.is_playing = false;
        song_playback.current_line_duration = Duration::ZERO;
        self.clear_channels();
    }

    fn audio_stream_sample_count_changed(&mut self, sample_count: usize) {
        assert_log!(self.song_playback.is_some());
        let Some(song_playback) = self.song_playback.as_mut() else {
            return;
        };

        assert_log!(self.step_output.is_some());
        let Some(step_output) = self.step_output.as_mut() else {
            return;
        };

        assert_log!(song_playback.line_signal.frame_rate == step_output.frame_rate);

        let frame_rate = step_output.frame_rate;

        song_playback.line_signal = signal::Owned::from_sample_count(sample_count, frame_rate);
        self.step_output = Some(signal::Owned::from_sample_count(sample_count, frame_rate));
    }

    fn initialize_audio(&mut self, frame_rate: f32) {
        assert_log!(frame_rate.is_normal());
        assert_log!(frame_rate > 0.0);
        self.song_playback = Some(model::playback::song::Playback {
            line_signal: signal::Owned::new(frame_rate),
            current_line: 0,
            current_line_duration: Duration::ZERO,
            line_duration: Duration::from_secs_f32(1.0 / self.line_per_second),
            is_playing: false,
        });

        self.step_output = Some(signal::Owned::new(frame_rate));
    }

    fn perform_playbacks_step(&mut self) {
        self.computed_frame_count = 0;

        assert_log!(self.step_output.is_some());
        let Some(step_output) = self.step_output.as_mut() else {
            return;
        };

        step_output.fill(Frame::default());

        assert_log!(self.song_playback.is_some());
        let Some(song_playback) = self.song_playback.as_mut() else {
            return;
        };

        if !song_playback.is_playing {
            for channel in self.channels.iter_mut() {
                channel.collect_mix_in(step_output.as_mut(), &self.instruments, self.global_volume);
            }
            self.computed_frame_count = step_output.as_ref().frame_count();
        } else {
            if song_playback.current_line as i32 >= self.patterns.channel_len {
                self.stop_song_playback();
                return;
            }

            let step_duration = step_output.as_ref().duration();
            let mut sub_step_start_duration = Duration::ZERO;

            while sub_step_start_duration < step_duration {
                let sub_step_duration = (song_playback.line_duration
                    - song_playback.current_line_duration)
                    .min(step_duration - sub_step_start_duration);

                let sub_step_end_duration = sub_step_start_duration + sub_step_duration;

                for channel in self.channels.iter_mut() {
                    channel.collect_mix_in(
                        step_output
                            .sub_signal_from_duration_mut(
                                sub_step_start_duration,
                                sub_step_end_duration,
                            )
                            .unwrap(),
                        &self.instruments,
                        self.global_volume,
                    );
                }

                sub_step_start_duration = sub_step_end_duration;

                song_playback.current_line_duration += sub_step_duration;
                if song_playback.current_line_duration >= song_playback.line_duration {
                    song_playback.current_line += 1;
                    if song_playback.current_line as i32 >= self.patterns.channel_len {
                        break;
                    }

                    song_playback.current_line_duration -= song_playback.line_duration;
                    for (line, channel) in self
                        .patterns
                        .current_pattern_row(song_playback.current_line)
                        .zip(&mut self.channels)
                    {
                        channel.setup_line(line);
                    }
                }
            }

            if self.follow_playing {
                self.patterns.current_row = song_playback.current_line as i32;
            }

            self.computed_frame_count =
                (sub_step_start_duration.as_secs_f32() * step_output.frame_rate) as usize;
        }
    }

    fn clear_channels(&mut self) {
        for channel in self.channels.iter_mut() {
            *channel = Channel::new();
        }
    }

    fn change_selected_instrument(&mut self, increment: i32) {
        self.instruments.increment_selected(increment);
    }
}
