use iced::event::Event;

use iced::{
    executor, font, subscription, Application, Command, Element, Font, Renderer, Settings,
    Subscription, Theme,
};

use iced::widget::scrollable;
use keybinding::KeyBindings;
use model::pattern::{HexDigit, NoteField, PatternCollection};
use model::{HexValue, Note, NoteValue, OctaveValue};
use rust_utils_macro::New;

use view::component::patterns::patterns_component;

use crate::audio::pcm_sample::PcmStereoSample;
use crate::model::pattern::ColumnLineElement;

mod audio;
mod keybinding;
mod model;
mod view;

pub fn main() -> iced::Result {
    let sound = PcmStereoSample::from_path("piano.wav").unwrap();
    dbg!(sound.sample_rate);
    Tracky::run(Settings::default())
}

#[derive(New)]
struct Tracky {
    pattern_collection: PatternCollection,
    pattern_scroll_id: scrollable::Id,
    default_octave: OctaveValue,
    keybindings: KeyBindings,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            pattern_collection: Default::default(),
            keybindings: Default::default(),
            default_octave: OctaveValue::new(5).unwrap(),
            pattern_scroll_id: scrollable::Id::unique(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    TrackyAction(keybinding::Action),
}

impl Tracky {
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

    pub fn get_current_octave(&self) -> OctaveValue {
        self.pattern_collection
            .current_line()
            .note_field
            .note
            .map(|n| n.octave)
            .unwrap_or(self.default_octave)
    }

    pub fn set_note(&mut self, note: Note) {
        let octave = self.get_current_octave();
        self.pattern_collection.current_line_mut().note_field =
            NoteField::new(Some(NoteValue::new(note, octave)));
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
        if let Some(note) = &mut self.pattern_collection.current_line_mut().note_field.note {
            note.octave = octave
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
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Default::default(), Command::none())
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
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        patterns_component(&self.pattern_collection, self.pattern_scroll_id.clone()).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccurred)
    }
}
