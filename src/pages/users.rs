// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};
use tui_textarea::TextArea;

use crate::{Action, Component};

pub struct Users<'a> {
    name: TextArea<'a>,
    password: TextArea<'a>,
}

impl<'a> Component for Users<'a> {
    fn render(&self, frame: &mut ratatui::prelude::Frame) {
        let area = Rect::new(
            frame.size().x + 2,
            frame.size().y + 2,
            frame.size().width - 4,
            frame.size().height - 4,
        );
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
            .spacing(1)
            .split(area);
        frame.render_widget(self.name.widget(), layout[0]);
        frame.render_widget(self.password.widget(), layout[1]);
    }

    fn update(&mut self, action: crate::Action) -> Option<crate::Action> {
        match action {
            Action::Key(k) => self.password.input_without_shortcuts(k),
            Action::Mouse(m) => self.password.input_without_shortcuts(m),
            _ => false,
        };

        None
    }
}

impl<'a> Default for Users<'a> {
    fn default() -> Self {
        Self::new()
    }
}

fn mkbox(title: &str) -> TextArea {
    let mut text = TextArea::default();
    text.set_cursor_line_style(Style::default());
    text.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Gray))
            .title(title)
            .title_style(Style::default().fg(Color::Gray)),
    );
    text
}

impl<'a> Users<'a> {
    pub fn new() -> Self {
        let mut text = mkbox(" Username ");
        text.set_cursor_style(Style::default());
        let mut password = mkbox(" Password ");
        password.set_mask_char('•');
        Self {
            name: mkbox(" Username "),
            password,
        }
    }
}
