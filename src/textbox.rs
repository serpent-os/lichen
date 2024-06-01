// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TextBox ...

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};
use tui_textarea::TextArea;

use crate::{Action, Component};

pub struct TextBox {
    area: TextArea<'static>,
}

impl TextBox {
    pub fn new(title: &str) -> Self {
        let mut text = TextArea::default();
        text.set_cursor_line_style(Style::default());
        text.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Gray))
                .title(title.to_string())
                .title_style(Style::default().fg(Color::Gray)),
        );

        Self { area: text }
    }
}

impl Component for TextBox {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        frame.render_widget(self.area.widget(), area)
    }

    fn update(&mut self, action: crate::Action) -> Option<crate::Action> {
        match action {
            Action::Key(k) => self.area.input(k),
            Action::Mouse(m) => self.area.input(m),
            _ => false,
        };
        None
    }
}
