use crate::{
    audio::{
        frame::YieldFrame,
        signal::StereoSignal,
        synthesis::{SawWave, SineWave, SquareWave},
        Pan, Volume,
    },
    model::{
        channel::{Channel, Instrument},
        midi::note_to_freq,
        pattern::{NoteFieldValue, PatternLine},
    },
};

impl Channel {
    pub fn setup_line(&mut self, line: &PatternLine) {
        if let Some(note) = line.note.value().cloned() {
            self.current_note = match note {
                NoteFieldValue::Note(note, octave) => Some((note, octave)),
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
        if let Some(instrument) = line.instrument.get_u8().map(|index| Instrument {
            index,
            current_phase: 0.0,
        }) {
            self.current_instrument = Some(instrument);
        };
    }

    pub fn collect_signal(&mut self, output_signal: &mut StereoSignal) {
        if let (
            Some((note, octave)),
            volume,
            Some(Instrument {
                index,
                current_phase,
            }),
        ) = (
            self.current_note,
            self.current_volume,
            &mut self.current_instrument,
        ) {
            let freq = note_to_freq(note, octave);
            let frames: &mut dyn YieldFrame = match index {
                0 => &mut SineWave,
                1 => &mut SawWave,
                2 => &mut SquareWave,
                _ => return,
            };

            frames.collect_in(
                output_signal,
                freq,
                volume.unwrap_or_default(),
                Pan::DEFAULT,
                current_phase,
            )
        } else {
            output_signal.fill(Default::default());
        }
    }
}
