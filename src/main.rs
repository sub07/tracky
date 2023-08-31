use std::time::{Duration, Instant};

use audio::audio_channel::handle_column;
use audio::generation::SineWaveDescriptor;
use audio::pcm_sample_player::PcmSamplePlayer;
use audio::Volume;

use iced::event::Event;
use iced::font::{Stretch, Weight};
use iced::widget::{scrollable, text};
use iced::{
    executor, font, subscription, time, Application, Command, Element, Font, Renderer, Settings,
    Subscription, Theme,
};

use keybinding::KeyBindings;
use model::pattern::{HexDigit, NoteField, PatternCollection};
use model::{HexValue, Note, NoteValue, OctaveValue};

use crate::audio::pcm_sample::PcmStereoSample;
use crate::model::pattern::ColumnLineElement;
use crate::view::component::patterns::patterns_component;

mod audio;
mod keybinding;
mod model;
mod view;

const MONOSPACED_FONT: Font = Font {
    family: font::Family::Name("Roboto Mono"),
    monospaced: true,
    stretch: Stretch::Normal,
    weight: Weight::Light,
};

pub fn main() -> iced::Result {
    let mut pcm_player = PcmSamplePlayer::new().unwrap();
    pcm_player.volume(Volume::new(0.1).unwrap());
    Tracky::run(Settings::with_flags(pcm_player))
}

struct Tracky {
    pattern_collection: PatternCollection,
    pattern_scroll_id: scrollable::Id,
    default_octave: OctaveValue,
    keybindings: KeyBindings,
    sample_player: PcmSamplePlayer,
    sine_hz: i32,
    sine_generator: SineWaveDescriptor,
    playing: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    TrackyAction(keybinding::Action),
    FontLoaded(Result<(), font::Error>),
    OnSineChanged(i32),
    Tick(Instant),
}

impl Tracky {
    fn new(sample_player: PcmSamplePlayer) -> Self {
        Self {
            pattern_collection: Default::default(),
            keybindings: Default::default(),
            default_octave: OctaveValue::new(5).unwrap(),
            pattern_scroll_id: scrollable::Id::unique(),
            sample_player,
            sine_hz: 100,
            sine_generator: SineWaveDescriptor::new(1.0),
            playing: false,
        }
    }

    pub fn convert_event_to_action(&self, event: Event) -> Option<keybinding::Action> {
        let input_context = self.pattern_collection.input_type();
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

    pub fn get_current_octave_or_default(&self) -> OctaveValue {
        match self.pattern_collection.current_line().note_field.note {
            Some(note) => match note {
                NoteValue::Cut => self.default_octave,
                NoteValue::Note(_, octave) => octave,
            },
            _ => self.default_octave,
        }
    }

    pub fn set_note(&mut self, note: Note) {
        let octave = self.get_current_octave_or_default();
        self.pattern_collection.current_line_mut().note_field =
            NoteField::new(Some(NoteValue::Note(note, octave)));
    }

    pub fn set_velocity(&mut self, hex_value: HexValue) {
        let velocity_digit_index = match self.pattern_collection.local_column_index() {
            3 => HexDigit::First,
            4 => HexDigit::Second,
            _ => panic!("Should not happen"),
        };

        self.pattern_collection
            .current_line_mut()
            .velocity_field
            .set_digit_hex(velocity_digit_index, hex_value)
    }

    pub fn set_instrument(&mut self, hex_value: HexValue) {
        let instr_digit_index = match self.pattern_collection.local_column_index() {
            5 => HexDigit::First,
            6 => HexDigit::Second,
            _ => panic!("Should not happen"),
        };

        self.pattern_collection
            .current_line_mut()
            .instrument_field
            .set_digit_hex(instr_digit_index, hex_value)
    }

    pub fn set_hex(&mut self, hex_value: HexValue) {
        match self.pattern_collection.local_column_index() {
            3 | 4 => self.set_velocity(hex_value),
            5 | 6 => self.set_instrument(hex_value),
            _ => {}
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        if let Some(NoteValue::Note(_, field_octave)) =
            &mut self.pattern_collection.current_line_mut().note_field.note
        {
            *field_octave = octave
        }
    }

    pub fn clear(&mut self) {
        match self.pattern_collection.local_column_index() {
            0 | 2 => {
                self.pattern_collection.current_line_mut().note_field = NoteField::default();
                self.pattern_collection
                    .current_line_mut()
                    .velocity_field
                    .clear();
                self.pattern_collection
                    .current_line_mut()
                    .instrument_field
                    .clear();
            }
            3 | 4 => self
                .pattern_collection
                .current_line_mut()
                .velocity_field
                .clear(),
            5 | 6 => self
                .pattern_collection
                .current_line_mut()
                .instrument_field
                .clear(),
            _ => {}
        }
    }

    pub fn move_cursor(
        &mut self,
        x: i32,
        y: i32,
    ) -> Command<<Tracky as iced::Application>::Message> {
        self.pattern_collection.cursor_x += x;
        self.pattern_collection.cursor_y += y;

        if self.pattern_collection.cursor_x % ColumnLineElement::LINE_LEN == 1 {
            self.pattern_collection.cursor_x += x;
        }

        self.pattern_collection.cursor_x = i32::rem_euclid(
            self.pattern_collection.cursor_x,
            ColumnLineElement::LINE_LEN
                * self.pattern_collection.current_pattern().columns.len() as i32,
        );
        self.pattern_collection.cursor_y = i32::rem_euclid(
            self.pattern_collection.cursor_y,
            self.pattern_collection.current_pattern().columns[0]
                .lines
                .len() as i32,
        );

        let cursor_x_column_index = self.pattern_collection.cursor_x / ColumnLineElement::LINE_LEN;

        return scrollable::snap_to(
            self.pattern_scroll_id.clone(),
            scrollable::RelativeOffset {
                x: cursor_x_column_index as f32
                    / (self.pattern_collection.current_pattern().columns.len() - 1) as f32,
                y: self.pattern_collection.cursor_y as f32
                    / (self.pattern_collection.current_pattern().columns[0]
                        .lines
                        .len()
                        - 1) as f32,
            },
        );
    }
}

impl Application for Tracky {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = PcmSamplePlayer;

    fn new(sample_player: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Tracky::new(sample_player),
            font::load(include_bytes!("../roboto_mono.ttf").as_slice()).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        "Tracky".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Some(action) = self.convert_event_to_action(event) {
                    return self.update(Message::TrackyAction(action));
                }
            }
            Message::TrackyAction(action) => match action {
                keybinding::Action::Note(note) => self.set_note(note),
                keybinding::Action::Hex(value) => self.set_hex(value),
                keybinding::Action::Octave(value) => self.set_octave(value),
                keybinding::Action::ClearUnit => self.clear(),
                keybinding::Action::Move(direction) => {
                    return self.move_cursor(direction.x(), direction.y())
                }
                keybinding::Action::InsertPattern => todo!(),
                keybinding::Action::NextPattern => todo!(),
                keybinding::Action::PreviousPattern => todo!(),
                keybinding::Action::TogglePlay => {
                    self.playing = !self.playing;
                    let bps = 6.0;
                    if self.playing {
                        let mut channel = PcmStereoSample::from_duration(
                            Duration::from_secs_f64(
                                (1.0 / bps)
                                    * self
                                        .pattern_collection
                                        .current_pattern()
                                        .column(0)
                                        .lines
                                        .len() as f64,
                            ),
                            self.sample_player.sample_rate,
                        );
                        handle_column(
                            bps,
                            &mut channel,
                            &self.pattern_collection.current_pattern().column(0),
                        );
                        self.sample_player.queue_pcm_samples(&channel).unwrap();
                    }
                }
                keybinding::Action::SetNoteCut => {
                    self.pattern_collection.current_line_mut().note_field.note =
                        Some(NoteValue::Cut)
                }
            },
            Message::FontLoaded(r) => {
                if let Err(e) = r {
                    panic!("{e:?}");
                }
            }
            Message::OnSineChanged(value) => {
                self.sine_hz = value;
            }
            Message::Tick(_now) => {
                // let pcm_samples = self.sine_generator.collect_for_duration(
                //     Duration::from_millis(10),
                //     self.sine_hz as f32,
                //     self.sample_player.sample_rate,
                // );
                // self.sample_player
                //     .queue_pcm_samples(&PcmStereoSample::from_frames(
                //         pcm_samples,
                //         self.sample_player.sample_rate,
                //     ))
                //     .unwrap();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        iced::widget::column![
            text(if self.playing { "playing" } else { "editing" }),
            patterns_component(&self.pattern_collection, self.pattern_scroll_id.clone()),
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            time::every(Duration::from_millis(10)).map(Message::Tick),
            subscription::events().map(Message::EventOccurred),
        ])
    }
}
