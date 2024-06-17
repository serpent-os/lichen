// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::time::Instant;

use crossterm::event::{KeyEvent, MouseEvent};
use tui_textarea::Input;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    RedrawRequested(Instant),
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

    pub fn input(self) -> Option<Input> {
        match self {
            Event::Key(key) => Some(crossterm::event::Event::Key(key).into()),
            Event::Mouse(mouse) => Some(crossterm::event::Event::Mouse(mouse).into()),
            Event::RedrawRequested(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Status {
    #[default]
    Ignored,
    Captured,
}
