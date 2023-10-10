use std::time::{Duration, Instant};

use audio::generation::SineWaveDescriptor;

use audio::model::signal::StereoSignal;
use audio::player::Player;
use audio::value_object::Volume;
use iced::event::Event;

use iced::widget::{scrollable, text};
use iced::{
    executor, font, subscription, time, Application, Command, Element, Renderer, Settings,
    Subscription, Theme,
};

use keybinding::KeyBindings;
use model::audio_channel::AudioChannel;
use model::field::value_object::OctaveValue;
use model::pattern::Patterns;

use crate::view::component::patterns::patterns_component;

mod audio;
mod keybinding;
mod model;
mod service;
mod view;

pub fn main() -> iced::Result {
    let player_sample_rate = Player::new().unwrap().sample_rate;
    Tracky::run(Settings {
        flags: player_sample_rate,
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
    audio_channels: Vec<AudioChannel>,
    mixer_output_signal: StereoSignal,
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
    fn new(patterns: Patterns, sample_rate: f32) -> Self {
        let nb_column = patterns.nb_column as usize;
        let mut audio_channels = Vec::with_capacity(nb_column);
        audio_channels.resize_with(nb_column, || AudioChannel::new(6.0, sample_rate));
        Self {
            patterns: patterns,
            keybindings: Default::default(),
            default_octave: OctaveValue::new(5).unwrap(),
            selected_instrument: 0,
            pattern_scroll_id: scrollable::Id::unique(),
            playing_state: PlayingState::Stopped,
            audio_channels,
            mixer_output_signal: StereoSignal::new(Duration::ZERO, sample_rate),
            sine_hz: 100,
            sine_generator: SineWaveDescriptor,
        }
    }
}

impl Application for Tracky {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = f32;

    fn new(sample_rate: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Tracky::new(Default::default(), sample_rate),
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
                        assert_eq!(self.patterns.nb_column, self.audio_channels.len() as u32);
                        
                        let mut player = audio::player::Player::new().unwrap();
                        player.volume(Volume::new(0.1).unwrap());
                        
                        for (column, audio_channel) in self.patterns.current_pattern().columns().zip(self.audio_channels.iter_mut()) {
                            audio_channel.handle_column(column);
                        }

                        let pattern_duration = self.audio_channels.iter().next().unwrap().signal().duration();
                        self.mixer_output_signal.ensure_duration(pattern_duration);

                        for audio_channel in self.audio_channels.iter() {
                            for ((mix_left, mix_right), (audio_channel_left, audio_channel_right)) in self.mixer_output_signal.frames.iter_mut().zip(audio_channel.signal().frames.iter()) {
                                *mix_left += *audio_channel_left;
                                *mix_right += *audio_channel_right;
                            }
                        }

                        // self.mixer_output_signal.write_signal_to_disk("sig.wav".into()).unwrap();
                        player.queue_signal(&self.mixer_output_signal).unwrap();
                        PlayingState::Playing(player)
                    }
                }
                keybinding::Action::SetNoteCut => self
                    .patterns
                    .current_line_mut()
                    .note
                    .set(model::field::NoteFieldValue::Cut),
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
            text(if let PlayingState::Playing(_) = &self.playing_state {
                "playing"
            } else {
                "editing"
            }),
            patterns_component(&self.patterns, self.pattern_scroll_id.clone()),
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
