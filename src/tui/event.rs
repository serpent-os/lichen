use crossterm::event::{KeyEvent, MouseEvent};
use serde::{Deserialize, Serialize};
use tui_textarea::Input;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

impl Event {
    pub fn from_crossterm(event: crossterm::event::Event) -> Option<Self> {
        match event {
            crossterm::event::Event::FocusGained => None,
            crossterm::event::Event::FocusLost => None,
            crossterm::event::Event::Key(key) => Some(Event::Key(key)),
            crossterm::event::Event::Mouse(mouse) => Some(Event::Mouse(mouse)),
            crossterm::event::Event::Paste(_) => None,
            crossterm::event::Event::Resize(_, _) => None,
        }
    }
}

impl From<Event> for Input {
    fn from(event: Event) -> Input {
        match event {
            Event::Key(key) => crossterm::event::Event::Key(key),
            Event::Mouse(mouse) => crossterm::event::Event::Mouse(mouse),
        }
        .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Status {
    #[default]
    Ignored,
    Captured,
}
