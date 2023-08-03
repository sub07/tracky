use iced::event::Event;
use iced::keyboard::KeyCode;
use iced::{
    executor, subscription, Application, Command, Element, Renderer, Settings, Subscription, Theme,
};

use iced_native::widget::scrollable::{self, Properties};

use keybinding::{KeyBindings, PatternInputType};
use model::pattern::{HexDigit, Pattern, PatternCollection, VelocityField};
use model::{HexValue, Note, NoteField, NoteValue, OctaveValue};
use rust_utils_macro::New;

use view::component::pattern::pattern_component;
use view::component::patterns::patterns_component;

use crate::model::pattern::{Column, ColumnLineElement};

mod keybinding;
mod model;
mod view;

pub fn main() -> iced::Result {
    Tracky::run(Settings {
        default_font: Some(include_bytes!("../font.ttf")),
        ..Default::default()
    })
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
        let pattern_input_type = self.pattern_collection.input_type();
        match event {
            Event::Keyboard(kb_event) => match kb_event {
                iced_native::keyboard::Event::KeyPressed {
                    key_code,
                    modifiers: _,
                } => {
                    if let Some(key_map) =
                        self.keybindings.context_bindings.get(&pattern_input_type)
                    {
                        if let Some(action) = key_map.get(&key_code) {
                            return Some(*action);
                        } else {
                            if let Some(global_keybinds) = self
                                .keybindings
                                .context_bindings
                                .get(&PatternInputType::Global)
                            {
                                if let Some(action) = global_keybinds.get(&key_code) {
                                    return Some(*action);
                                }
                            }
                        }
                    }
                }
                iced_native::keyboard::Event::KeyReleased {
                    key_code: _,
                    modifiers: _,
                } => {}
                iced_native::keyboard::Event::CharacterReceived(_) => {}
                iced_native::keyboard::Event::ModifiersChanged(_) => {}
            },
            Event::Mouse(_) => {}
            Event::Window(_) => {}
            Event::Touch(_) => {}
        }
        None
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

    pub fn set_hex(&mut self, hex_value: HexValue) {
        match self.pattern_collection.local_column_index() {
            3 | 4 => self.set_velocity(hex_value),
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
                self.pattern_collection.current_line_mut().velocity_field = VelocityField::default()
            }
            3 | 4 => {
                self.pattern_collection.current_line_mut().velocity_field = VelocityField::default()
            }
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
