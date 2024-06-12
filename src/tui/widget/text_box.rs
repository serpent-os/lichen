// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TextBox encapsulates tui-textarea with lichen styling

use std::cell::RefCell;

use crossterm::event::{KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Position, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::tui::{event, layout, widget, Element, Event, Layout, Shell, Widget};

pub fn text_box<'a>(state: &'a State) -> TextBox<'a> {
    TextBox::new(state)
}

#[derive(Default)]
pub struct State(RefCell<Inner>);

#[derive(Default)]
pub struct Inner {
    area: TextArea<'static>,
    hovered: bool,
    id: widget::Id,
}

impl State {
    pub fn lines(&self) -> Vec<String> {
        self.0.borrow().area.lines().to_vec()
    }

    pub fn reset(&mut self) {
        *self = State::default();
    }
}

pub struct TextBox<'a> {
    state: &'a State,
    style: Box<dyn Fn(Status, bool) -> Stylesheet + 'a>,
    title: Option<String>,
}

impl<'a> TextBox<'a> {
    /// Return a new TextBox with the given title
    pub fn new(state: &'a State) -> Self {
        Self {
            state,
            style: Box::new(|_, _| Stylesheet::default()),
            title: None,
        }
    }

    /// Set as a password field
    pub fn hide_chars(self) -> Self {
        self.state.0.borrow_mut().area.set_mask_char('•');
        self
    }

    pub fn style(self, f: impl Fn(Status, bool) -> Stylesheet + 'a) -> Self {
        Self {
            style: Box::new(f),
            ..self
        }
    }

    pub fn title(self, title: impl ToString) -> Self {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }
}

impl<'a, Message> Widget<Message> for TextBox<'a> {
    fn height(&self, _width: u16) -> Constraint {
        Constraint::Length((self.state.0.borrow().area.lines().len() as u16).max(1) + 2)
    }

    fn layout(&self, available: Rect) -> Layout {
        Layout {
            area: Rect {
                height: ((self.state.0.borrow().area.lines().len() as u16).max(1) + 2)
                    .min(available.height),
                ..available
            },
            children: vec![],
        }
    }

    fn update(
        &mut self,
        layout: &Layout,
        event: Event,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        let mut state = self.state.0.borrow_mut();
        let focused = Some(state.id) == shell.focused();

        match event {
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Moved {
                    let pos = Position::new(mouse.column, mouse.row);
                    let prev = state.hovered;

                    state.hovered = layout.area.contains(pos);

                    if state.hovered != prev {
                        shell.request_redraw();
                    }
                } else if mouse.kind == MouseEventKind::Up(MouseButton::Left) {
                    let pos = Position::new(mouse.column, mouse.row);

                    if layout.area.contains(pos) {
                        shell.focus(state.id);
                        return event::Status::Captured;
                    } else if focused {
                        shell.unfocus();
                        return event::Status::Captured;
                    }
                }
            }
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc if focused => {
                    shell.unfocus();
                    return event::Status::Captured;
                }
                KeyCode::Tab | KeyCode::BackTab | KeyCode::Enter => return event::Status::Ignored,
                _ => {}
            },
            _ => {}
        }

        if focused {
            if let Some(input) = event.input() {
                if state.area.input(input) {
                    shell.request_redraw();
                    return event::Status::Captured;
                }
            }
        }

        event::Status::Ignored
    }

    fn render(
        &self,
        frame: &mut ratatui::prelude::Frame,
        layout: &Layout,
        focused: Option<widget::Id>,
    ) {
        let mut state = self.state.0.borrow_mut();
        let focused = Some(state.id) == focused;

        let status = if focused {
            Status::Active
        } else if state.hovered {
            Status::Hovered
        } else {
            Status::Inactive
        };

        let style = (self.style)(status, state.area.mask_char().is_some());

        state.area.set_style(style.area);
        state.area.set_cursor_style(style.cursor);

        let mut block = Block::default()
            .border_style(style.borders)
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        if let Some(title) = &self.title {
            block = block.title(title.as_str());
        }

        frame.render_widget(block, layout.area);

        frame.render_widget(
            state.area.widget(),
            layout::pad_rect(layout.area, Padding::new(2, 2, 1, 1)),
        );
    }

    fn flatten(&self) -> Vec<widget::Info> {
        let state = &self.state.0.borrow();

        Some(widget::Info::focusable(state.id))
            .into_iter()
            .collect()
    }
}

impl<'a, Message> From<TextBox<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: TextBox<'a>) -> Self {
        Element::new(value)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Stylesheet {
    pub area: Style,
    pub cursor: Style,
    pub borders: Style,
}

pub enum Status {
    Inactive,
    Hovered,
    Active,
}
