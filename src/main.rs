#![feature(const_fn_floating_point_arithmetic)]
#![feature(duration_constants)]
#![feature(let_chains)]

use std::time::{Duration, Instant};

use audio::audio_channel::AudioChannel;
use audio::value_object::Volume;
use iced::event::Event;

use iced::widget::{scrollable, text};
use iced::{
    executor, font, subscription, time, Application, Command, Element, Renderer, Settings,
    Subscription, Theme,
};

use iter_tools::Itertools;
use keybinding::KeyBindings;

use model::field::value_object::OctaveValue;
use model::pattern::Patterns;

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
    Playing {
        player: audio::player::Player,
        current_line: f32,
        last_time: Instant,
        should_handle_next_line: bool,
    },
}

struct Tracky {
    patterns: Patterns,
    pattern_scroll_id: scrollable::Id,
    default_octave: OctaveValue,
    selected_instrument: u8,
    keybindings: KeyBindings,
    playing_state: PlayingState,
    line_per_minute: f32,
}

#[derive(Debug)]
enum Message {
    EventOccurred(Event),
    TrackyAction(keybinding::Action),
    FontLoaded(Result<(), font::Error>),
    Tick(Instant),
}

impl Tracky {
    fn new(patterns: Patterns) -> Self {
        Self {
            patterns,
            keybindings: Default::default(),
            default_octave: OctaveValue::OCTAVE_5,
            selected_instrument: 3,
            pattern_scroll_id: scrollable::Id::unique(),
            playing_state: PlayingState::Stopped,
            line_per_minute: 120.0,
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
                    self.playing_state = if let PlayingState::Playing { .. } = self.playing_state {
                        PlayingState::Stopped
                    } else {
                        let mut player = audio::player::Player::new().unwrap();
                        player.volume(Volume::new(0.4).unwrap());
                        self.patterns.cursor_y = -1;
                        PlayingState::Playing {
                            player,
                            current_line: 0.0,
                            last_time: Instant::now(),
                            should_handle_next_line: false,
                        }
                    }
                }
                keybinding::Action::SetNoteCut => self
                    .patterns
                    .current_line_mut()
                    .note
                    .set(model::field::NoteFieldValue::Cut),
                keybinding::Action::ModifyDefaultOctave(increment) => {
                    if let Some(new_default_octave) =
                        OctaveValue::new(self.default_octave.value() + increment)
                    {
                        self.default_octave = new_default_octave;
                    }
                }
            },
            Message::FontLoaded(font_loading_result) => {
                if let Err(e) = font_loading_result {
                    panic!("{e:?}");
                }
            }
            Message::Tick(now) => {
                self.play_line(now);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        iced::widget::column![
            iced::widget::row![
                text(if let PlayingState::Playing { .. } = &self.playing_state {
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
