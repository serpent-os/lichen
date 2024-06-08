// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TextBox encapsulates tui-textarea with lichen styling

use std::cell::RefCell;

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::{theme, widget::State, Action, Widget};

pub struct TextBox<'a> {
    area: RefCell<TextArea<'a>>,
    state: RefCell<State>,
}

impl<'a> TextBox<'a> {
    /// Return a new TextBox with the given title
    pub fn new(title: impl AsRef<str>) -> Self {
        let mut text = TextArea::default();
        text.set_cursor_line_style(Style::default());
        text.set_style(Style::default());
        text.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::current().color_inactive))
                .title(title.as_ref().to_string())
                .title_style(Style::default().fg(theme::current().color_inactive))
                .padding(Padding::symmetric(1, 0)),
        );

        Self {
            area: RefCell::new(text),
            state: RefCell::new(State::NONE),
        }
    }

    /// Set as a password field
    pub fn set_hide_chars(&self) {
        self.area.borrow_mut().set_mask_char('•')
    }

    /// Update text style based on state
    fn style_from_state(&self) {
        let mut area = self.area.borrow_mut();
        let state = self.state.borrow_mut();
        let style = if state.contains(State::ACTIVE) {
            area.set_cursor_style(Style::default().reversed());
            Style::default()
        } else {
            area.set_cursor_style(Style::default());
            Style::default().fg(theme::current().color_inactive)
        };
        let styled = if area.mask_char().is_some() {
            style.bold()
        } else {
            style
        };
        area.set_style(styled);
    }

    // Update block style based on state
    fn block_from_state(&self) {
        let state = self.state.borrow();
        let mut area = self.area.borrow_mut();
        let block = if state.contains(State::ACTIVE) {
            area.block()
                .unwrap()
                .clone()
                .border_style(Style::default().fg(theme::current().color_selection))
                .title_style(Style::default().fg(theme::current().color_inactive))
        } else if state.contains(State::HIGHLIGHT) {
            area.block()
                .unwrap()
                .clone()
                .border_style(Style::default().fg(theme::current().color_highlight))
                .title_style(Style::default().fg(theme::current().color_inactive))
        } else {
            area.block()
                .unwrap()
                .clone()
                .border_style(Style::default().fg(theme::current().color_inactive))
                .title_style(Style::default().fg(theme::current().color_inactive))
        };
        area.set_block(block);
    }
}

impl<'a> Widget for TextBox<'a> {
    /// Render to bounds
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        frame.render_widget(self.area.borrow().widget(), area)
    }

    // Update state
    fn update(&self, action: crate::Action) -> Option<crate::Action> {
        let mut area = self.area.borrow_mut();
        match action {
            Action::Key(k) => area.input(k),
            Action::Mouse(m) => area.input(m),
            _ => false,
        };
        None
    }

    /// Return our state
    fn state(&self) -> State {
        *self.state.borrow()
    }

    /// Push a new state to the set
    fn push_state(&self, st: crate::widget::State) {
        self.state.borrow_mut().insert(st);
        self.block_from_state();
        self.style_from_state();
    }

    /// Pop a state from the set
    fn pop_state(&self, st: crate::widget::State) {
        self.state.borrow_mut().remove(st);
        self.block_from_state();
        self.style_from_state();
    }
}
