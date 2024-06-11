// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TextBox encapsulates tui-textarea with lichen styling

use std::cell::RefCell;

use crossterm::event::MouseEventKind;
use ratatui::{
    layout::{Constraint, Position, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::tui::{event, layout, Element, Event, Layout, Shell, Widget};

pub fn text_box<'a>(state: &'a State) -> TextBox<'a> {
    TextBox::new(state)
}

#[derive(Default)]
pub struct State(RefCell<Inner>);

#[derive(Default)]
pub struct Inner {
    area: TextArea<'static>,
    hovered: bool,
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
    style: Box<dyn Fn(Status) -> Stylesheet + 'a>,
    title: Option<String>,
}

impl<'a> TextBox<'a> {
    /// Return a new TextBox with the given title
    pub fn new(state: &'a State) -> Self {
        Self {
            state,
            style: Box::new(|_| Stylesheet::default()),
            title: None,
        }
    }

    /// Set as a password field
    pub fn hide_chars(self) -> Self {
        self.state.0.borrow_mut().area.set_mask_char('•');
        self
    }

    pub fn style(self, f: impl Fn(Status) -> Stylesheet + 'a) -> Self {
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
    fn width(&self) -> Constraint {
        Constraint::Fill(1)
    }

    fn height(&self) -> Constraint {
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
        if let Event::Mouse(mouse) = event {
            let prev = self.state.0.borrow_mut().hovered;
            // TODO: Why isn't moved working?
            if mouse.kind == MouseEventKind::Moved {
                let pos = Position::new(mouse.column, mouse.row);
                self.state.0.borrow_mut().hovered = layout.area.contains(pos);

                if self.state.0.borrow_mut().hovered != prev {
                    shell.request_redraw();
                }
            }
        }

        // TODO: Focus
        if self.state.0.borrow_mut().area.input(event) {
            shell.request_redraw();
            // event::Status::Captured
            event::Status::Ignored
        } else {
            event::Status::Ignored
        }
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout) {
        let status = if self.state.0.borrow().hovered {
            Status::Hovered
        } else {
            // TODO: Focus tracking
            Status::Inactive
        };

        let style = (self.style)(status);

        let mut block = Block::default()
            .border_style(style.borders)
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        if let Some(title) = &self.title {
            block = block.title(title.as_str());
        }

        frame.render_widget(block, layout.area);

        frame.render_widget(
            self.state.0.borrow().area.widget(),
            layout::pad_rect(layout.area, Padding::new(2, 2, 1, 1)),
        );
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
    pub borders: Style,
}

pub enum Status {
    Inactive,
    Hovered,
    Active,
}
