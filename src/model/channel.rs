use std::iter;

use crate::audio::{signal, Pan, Volume};

use super::{
    instrument::Instruments,
    midi::note_to_freq,
    pattern::{NoteFieldValue, NoteName, OctaveValue, PatternLine},
};

#[derive(Clone, Debug)]
pub struct PlayingInstrument {
    pub phase: f32,
    pub index: u8,
}

#[derive(Clone, Debug)]
pub struct Channel {
    pub current_note: Option<(NoteName, OctaveValue)>,
    pub current_volume: Option<Volume>,
    pub current_instrument: Option<PlayingInstrument>,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            current_note: None,
            current_volume: None,
            current_instrument: None,
        }
    }

    pub fn setup_line(&mut self, line: &PatternLine) {
        if let Some(note) = line.note.value().cloned() {
            self.current_note = match note {
                NoteFieldValue::Note(note, octave) => {
                    if let Some((_, playing_instrument)) =
                        self.current_note.zip(self.current_instrument.as_mut())
                    {
                        playing_instrument.phase = 0.0;
                    }
                    Some((note, octave))
                }
                NoteFieldValue::Cut => {
                    self.current_volume = None;
                    self.current_instrument = None;
                    None
                }
            };
        }
        if let Some(volume) = line.velocity.get_percentage().map(Volume::new_unchecked) {
            self.current_volume = Some(volume);
        };
        if let Some(new_index) = line.instrument.get_u8() {
            if self
                .current_instrument
                .as_ref()
                .is_none_or(|current_instrument| current_instrument.index != new_index)
            {
                self.current_instrument = Some(PlayingInstrument {
                    phase: 0.0,
                    index: new_index,
                });
            }
        };
    }

    pub fn collect_mix_in(
        &mut self,
        mut output_signal: signal::stereo::Mut,
        instruments: &Instruments,
        global_volume: Volume,
    ) {
        if let (Some((note, octave)), volume, Some(PlayingInstrument { index, phase })) = (
            self.current_note,
            self.current_volume,
            &mut self.current_instrument,
        ) {
            let freq = note_to_freq(note, octave);
            let frame_rate = output_signal.frame_rate;

            if let Some(instrument) = instruments.get(*index) {
                for (output, generated) in output_signal.iter_mut().zip(iter::repeat_with(|| {
                    instrument.next_frame(
                        freq,
                        volume.unwrap_or_default() * global_volume,
                        Pan::DEFAULT,
                        phase,
                        frame_rate,
                    )
                })) {
                    *output += generated;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::model::pattern::{
        Field, HexDigit, NoteFieldValue, PatternLine, PatternLineDescriptor,
    };

    use super::*;

    fn get_channel() -> Channel {
        Channel::new()
    }

    // example line: "C#5 5F 03"
    fn make_line(line: &'static str) -> PatternLine {
        assert_eq!(
            line.len(),
            PatternLineDescriptor::LINE_LEN as usize + PatternLineDescriptor::COUNT - 1
        );
        fn parse_note(note: &'static str) -> Option<NoteFieldValue> {
            assert_eq!(PatternLineDescriptor::Note.field_len(), note.len());
            if note == "CUT" {
                Some(NoteFieldValue::Cut)
            } else if note == "..." {
                None
            } else {
                let (note_1, note_2, octave) = {
                    let mut chars = note.chars();
                    let n1 = chars.next().unwrap();
                    let n2 = chars.next().unwrap();
                    let o = chars.next().unwrap();
                    (n1, n2, o)
                };
                assert!(note_1 as i32 >= 65 && note_1 as i32 <= 71);
                assert!(note_2 == '#' || note_2 == '-');
                assert!(octave as i32 >= 48 && octave as i32 <= 57);

                let is_sharp = note_2 == '#';

                let note_name = if is_sharp {
                    match note_1 {
                        'C' => NoteName::CSharp,
                        'D' => NoteName::DSharp,
                        'F' => NoteName::FSharp,
                        'G' => NoteName::GSharp,
                        'A' => NoteName::ASharp,
                        _ => panic!("illegal value"),
                    }
                } else {
                    match note_1 {
                        'C' => NoteName::C,
                        'D' => NoteName::D,
                        'E' => NoteName::E,
                        'F' => NoteName::F,
                        'G' => NoteName::G,
                        'A' => NoteName::A,
                        'B' => NoteName::B,
                        _ => panic!("illegal value"),
                    }
                };

                let octave = match octave {
                    '0' => OctaveValue::OCTAVE_0,
                    '1' => OctaveValue::OCTAVE_1,
                    '2' => OctaveValue::OCTAVE_2,
                    '3' => OctaveValue::OCTAVE_3,
                    '4' => OctaveValue::OCTAVE_4,
                    '5' => OctaveValue::OCTAVE_5,
                    '6' => OctaveValue::OCTAVE_6,
                    '7' => OctaveValue::OCTAVE_7,
                    '8' => OctaveValue::OCTAVE_8,
                    '9' => OctaveValue::OCTAVE_9,
                    _ => unreachable!(),
                };

                Some(NoteFieldValue::Note(note_name, octave))
            }
        }

        fn parse_hex(hex: &'static str) -> Option<(HexDigit, HexDigit)> {
            assert_eq!(2, hex.len());

            if hex == ".." {
                None
            } else {
                let mut chars = hex.chars();
                let hex_1 = chars.next().unwrap();
                let hex_2 = chars.next().unwrap();

                fn parse_digit(c: char) -> HexDigit {
                    match c {
                        'A' => HexDigit::HEX_A,
                        'B' => HexDigit::HEX_B,
                        'C' => HexDigit::HEX_C,
                        'D' => HexDigit::HEX_D,
                        'E' => HexDigit::HEX_E,
                        'F' => HexDigit::HEX_F,
                        '0' => HexDigit::HEX_0,
                        '1' => HexDigit::HEX_1,
                        '2' => HexDigit::HEX_2,
                        '3' => HexDigit::HEX_3,
                        '4' => HexDigit::HEX_4,
                        '5' => HexDigit::HEX_5,
                        '6' => HexDigit::HEX_6,
                        '7' => HexDigit::HEX_7,
                        '8' => HexDigit::HEX_8,
                        '9' => HexDigit::HEX_9,
                        _ => panic!("Illegal character"),
                    }
                }
                Some((parse_digit(hex_1), parse_digit(hex_2)))
            }
        }

        let note = parse_note(&line[0..3]).map_or_else(Field::empty, Field::new);
        let velocity = parse_hex(&line[4..6]).map_or_else(Field::empty, Field::new);
        let instrument = parse_hex(&line[7..9]).map_or_else(Field::empty, Field::new);
        PatternLine {
            note,
            velocity,
            instrument,
        }
    }

    #[test]
    #[should_panic]
    fn test_make_line_1() {
        make_line("");
    }

    #[test]
    fn test_make_line_2() {
        assert_eq!(
            PatternLine {
                ..Default::default()
            },
            make_line("... .. ..")
        );
    }

    #[test]
    #[should_panic]
    fn test_make_line_3() {
        make_line("C.5 .. ..");
    }

    #[test]
    fn test_make_line_4() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(NoteName::C, OctaveValue::OCTAVE_2)),
                ..Default::default()
            },
            make_line("C-2 .. ..")
        );
    }

    #[test]
    fn test_make_line_5() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(
                    NoteName::DSharp,
                    OctaveValue::OCTAVE_3
                )),
                ..Default::default()
            },
            make_line("D#3 .. ..")
        );
    }

    #[test]
    #[should_panic]
    fn test_make_line_6() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(
                    NoteName::DSharp,
                    OctaveValue::OCTAVE_3
                )),
                ..Default::default()
            },
            make_line("D#3 F. ..")
        );
    }

    #[test]
    fn test_make_line_7() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(
                    NoteName::DSharp,
                    OctaveValue::OCTAVE_3
                )),
                velocity: Field::new((HexDigit::HEX_2, HexDigit::HEX_4)),
                ..Default::default()
            },
            make_line("D#3 24 ..")
        );
    }

    #[test]
    fn test_make_line_8() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(
                    NoteName::DSharp,
                    OctaveValue::OCTAVE_3
                )),
                velocity: Field::new((HexDigit::HEX_2, HexDigit::HEX_4)),
                instrument: Field::new((HexDigit::HEX_A, HexDigit::HEX_B)),
            },
            make_line("D#3 24 AB")
        );
    }

    #[test]
    fn test_make_line_9() {
        assert_eq!(
            PatternLine {
                note: Field::empty(),
                velocity: Field::new((HexDigit::HEX_2, HexDigit::HEX_4)),
                instrument: Field::new((HexDigit::HEX_A, HexDigit::HEX_B)),
            },
            make_line("... 24 AB")
        );
    }

    #[test]
    fn test_make_line_10() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(
                    NoteName::DSharp,
                    OctaveValue::OCTAVE_3
                )),
                velocity: Field::empty(),
                instrument: Field::new((HexDigit::HEX_A, HexDigit::HEX_B)),
            },
            make_line("D#3 .. AB")
        );
    }

    #[test]
    fn test_make_line_11() {
        assert_eq!(
            PatternLine {
                note: Field::new(NoteFieldValue::Note(
                    NoteName::DSharp,
                    OctaveValue::OCTAVE_3
                )),
                velocity: Field::new((HexDigit::HEX_A, HexDigit::HEX_B)),
                instrument: Field::empty(),
            },
            make_line("D#3 AB ..")
        );
    }

    #[test]
    #[should_panic]
    fn test_make_line_12() {
        make_line("...........");
    }

    #[test]
    #[should_panic]
    fn test_make_line_13() {
        make_line("..");
    }

    fn assert_channel_state(
        note: Option<(NoteName, OctaveValue)>,
        volume: Option<f32>,
        instrument: Option<u8>,
        channel: &Channel,
    ) {
        assert_eq!(note, channel.current_note);
        assert_eq!(volume.is_some(), channel.current_volume.is_some());
        if volume.is_some() && channel.current_volume.is_some() {
            approx::assert_relative_eq!(
                volume.unwrap(),
                channel.current_volume.unwrap().value(),
                epsilon = 0.001
            );
        }
        assert_eq!(
            instrument,
            channel.current_instrument.clone().map(|i| i.index)
        );
    }

    #[test]
    fn test_channel_state_when_doing_nothing() {
        let channel = get_channel();

        assert_channel_state(None, None, None, &channel);
    }

    #[test]
    fn test_channel_play_note() {
        let mut channel = get_channel();

        channel.setup_line(&make_line("C#5 .. .."));

        assert_channel_state(
            Some((NoteName::CSharp, OctaveValue::OCTAVE_5)),
            None,
            None,
            &channel,
        );
    }

    #[test]
    fn test_channel_play_velocity() {
        let mut channel = get_channel();

        channel.setup_line(&make_line("... 5F .."));

        assert_channel_state(None, Some(0.372_549), None, &channel);
    }

    #[test]
    fn test_channel_play_instrument() {
        let mut channel = get_channel();

        channel.setup_line(&make_line("... .. 03"));

        assert_channel_state(None, None, Some(3), &channel);
    }

    #[test]
    fn test_channel_play_full_line() {
        let mut channel = get_channel();

        channel.setup_line(&make_line("C#5 5F 03"));

        assert_channel_state(
            Some((NoteName::CSharp, OctaveValue::OCTAVE_5)),
            Some(0.372_549),
            Some(3),
            &channel,
        );
    }

    #[test]
    fn test_channel_play_multiple_line_1() {
        let mut channel = get_channel();

        channel.setup_line(&make_line("C#5 5F 03"));
        channel.setup_line(&make_line("... .. .."));

        assert_channel_state(
            Some((NoteName::CSharp, OctaveValue::OCTAVE_5)),
            Some(0.372_549),
            Some(3),
            &channel,
        );

        channel.setup_line(&make_line("... .. .."));
        channel.setup_line(&make_line("... .. .."));
        channel.setup_line(&make_line("... .. .."));

        assert_channel_state(
            Some((NoteName::CSharp, OctaveValue::OCTAVE_5)),
            Some(0.372_549),
            Some(3),
            &channel,
        );
    }

    #[test]
    fn test_channel_play_multiple_line_2() {
        let mut channel = get_channel();

        channel.setup_line(&make_line("C#5 5F 03"));
        channel.setup_line(&make_line("... .. .."));
        channel.setup_line(&make_line("D-8 .. .."));

        assert_channel_state(
            Some((NoteName::D, OctaveValue::OCTAVE_8)),
            Some(0.372_549),
            Some(3),
            &channel,
        );

        channel.setup_line(&make_line("... 4B .."));

        assert_channel_state(
            Some((NoteName::D, OctaveValue::OCTAVE_8)),
            Some(0.294117),
            Some(3),
            &channel,
        );

        channel.setup_line(&make_line("... .. A5"));

        assert_channel_state(
            Some((NoteName::D, OctaveValue::OCTAVE_8)),
            Some(0.294117),
            Some(165),
            &channel,
        );
    }
}
