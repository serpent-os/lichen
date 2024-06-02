// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Button ...

use ratatui::{
    layout::Constraint,
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{component::Orientation, Component, State};

pub struct Button {
    contents: String,
}

impl Component for Button {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let line = Line::from(self.contents.as_str());
        let draw = Paragraph::new(line)
            .block(
                Block::default()
                    .padding(Padding::symmetric(1, 0))
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .centered();

        frame.render_widget(draw, area)
    }

    fn update(&mut self, action: crate::Action) -> Option<crate::Action> {
        None
    }

    fn state(&self) -> State {
        State::NONE
    }

    fn push_state(&mut self, _: State) {}

    fn pop_state(&mut self, _: State) {}

    fn constraints(&self, orient: Orientation) -> Constraint {
        match orient {
            Orientation::Horizontal => Constraint::Max((self.contents.chars().count() + 6) as u16),
            Orientation::Vertical => Constraint::Max(3),
        }
    }
}

impl Button {
    /// Create a new button with the given text
    pub fn new(contents: impl AsRef<str>) -> Self {
        Self {
            contents: contents.as_ref().to_string(),
        }
    }
}
