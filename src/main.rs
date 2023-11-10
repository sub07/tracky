use std::time::{Duration, Instant};

use audio::generation::SineWaveDescriptor;

use audio::value_object::Volume;
use iced::event::Event;

use iced::widget::{scrollable, text};
use iced::{
    executor, font, subscription, time, Application, Command, Element, Renderer, Settings,
    Subscription, Theme,
};

use keybinding::KeyBindings;

use model::field::value_object::OctaveValue;
use model::pattern::Patterns;

use crate::service::audio_channel;
use crate::view::component::patterns::patterns_component;

mod audio;
mod keybinding;
mod model;
mod service;
mod view;

pub fn main() -> iced::Result {
    Tracky::run(Settings {
        ..Settings::default()
    })
}

pub enum PlayingState {
    Stopped,
    Playing(audio::player::Player),
}

struct Tracky {
    patterns: Patterns,
    pattern_scroll_id: scrollable::Id,
    default_octave: OctaveValue,
    selected_instrument: u8,
    keybindings: KeyBindings,
    playing_state: PlayingState,
    sine_hz: i32,
    sine_generator: SineWaveDescriptor,
}

#[derive(Debug)]
enum Message {
    EventOccurred(Event),
    TrackyAction(keybinding::Action),
    FontLoaded(Result<(), font::Error>),
    OnSineChanged(i32),
    Tick(Instant),
}

impl Tracky {
    fn new(patterns: Patterns) -> Self {
        Self {
            patterns: patterns,
            keybindings: Default::default(),
            default_octave: OctaveValue::OCTAVE_5,
            selected_instrument: 3,
            pattern_scroll_id: scrollable::Id::unique(),
            playing_state: PlayingState::Stopped,
            sine_hz: 100,
            sine_generator: SineWaveDescriptor,
        }
    }
}

impl Application for Tracky {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Tracky::new(Default::default()),
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
                keybinding::Action::Note(note) => self.set_note_name(note),
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
                    self.playing_state = if let PlayingState::Playing(_) = self.playing_state {
                        PlayingState::Stopped
                    } else {
                        let mut player = audio::player::Player::new().unwrap();
                        player.volume(Volume::new(0.5).unwrap());

                        let pattern_audio =
                            audio_channel::handle_patterns(&self.patterns, player.sample_rate, 6.0);

                        // self.mixer_output_signal.write_signal_to_disk("sig.wav".into()).unwrap();
                        player.queue_signal(&pattern_audio).unwrap();
                        PlayingState::Playing(player)
                    }
                }
                keybinding::Action::SetNoteCut => self
                    .patterns
                    .current_line_mut()
                    .note
                    .set(model::field::NoteFieldValue::Cut),
                keybinding::Action::ModifyDefaultOctave(increment) => {
                    let default_octave = self.default_octave.value() as i32;
                    if let Some(new_default_octave) = OctaveValue::new(default_octave + increment) {
                        self.default_octave = new_default_octave;
                    }
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
                if let PlayingState::Playing(player) = &mut self.playing_state {
                    if player.is_finished() {
                        self.playing_state = PlayingState::Stopped;
                    }
                }
                // let frames = Samples::<audio::frame::Mono>::collect_for_duration(
                //     &mut self.sine_generator,
                //     Duration::from_millis(10),
                //     self.sine_hz as f32,
                //     self.sample_player.sample_rate,
                // );
                // self.sample_player
                //     .queue_pcm_signal(&BufferSignal::from_frames(
                //         frames,
                //         self.sample_player.sample_rate,
                //     ))
                //     .unwrap();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        iced::widget::column![
            iced::widget::row![
                text(if let PlayingState::Playing(_) = &self.playing_state {
                    "playing"
                } else {
                    "editing"
                }),
                text(format!("Octave: {}", self.default_octave.value()))
            ]
            .spacing(16.0),
            patterns_component(&self.patterns, self.pattern_scroll_id.clone()),
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            time::every(Duration::from_millis(100)).map(Message::Tick),
            subscription::events().map(Message::EventOccurred),
        ])
    }
}
