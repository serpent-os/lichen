// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Trivial button widget

use std::cell::RefCell;

use crossterm::event::{KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Position, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding},
};

use crate::{event, layout, widget, Element, Event, Layout, Shell, Widget};

pub fn button<'a, Message>(state: &'a State, content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    Button {
        state,
        content: content.into(),
        padding: Padding::new(1, 1, 0, 0),
        on_press: None,
        style: Box::new(|_| Stylesheet::default()),
    }
}

#[derive(Default)]
pub struct State(RefCell<Inner>);

#[derive(Default)]
struct Inner {
    id: widget::Id,
    hovered: bool,
    pressed: bool,
}

pub struct Button<'a, Message> {
    content: Element<'a, Message>,
    padding: Padding,
    on_press: Option<Message>,
    state: &'a State,
    style: Box<dyn Fn(Status) -> Stylesheet + 'a>,
}

impl<'a, Message> Button<'a, Message> {
    pub fn on_press(self, message: Message) -> Self {
        Self {
            on_press: Some(message),
            ..self
        }
    }

    pub fn padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    pub fn style(self, f: impl Fn(Status) -> Stylesheet + 'a) -> Self {
        Self {
            style: Box::new(f),
            ..self
        }
    }
}

impl<'a, Message> Widget<Message> for Button<'a, Message>
where
    Message: Clone + 'a,
{
    fn width(&self, height: u16) -> Constraint {
        let left = self.padding.left + 1;
        let right = self.padding.right + 1;

        layout::pad_constraint(self.content.width(height), left + right)
    }

    fn height(&self, width: u16) -> Constraint {
        let top = self.padding.top + 1;
        let bottom = self.padding.bottom + 1;

        layout::pad_constraint(self.content.height(width), top + bottom)
    }

    fn layout(&self, available: Rect) -> Layout {
        let mut padding = self.padding;
        padding.left += 1;
        padding.right += 1;
        padding.top += 1;
        padding.bottom += 1;

        let inner = layout::pad_rect(available, padding);

        let content = self.content.layout(inner);

        let area = available.clamp(Rect {
            width: content.area.width + padding.left + padding.right,
            height: content.area.height + padding.top + padding.bottom,
            ..available
        });

        Layout {
            area,
            children: vec![content],
        }
    }

    fn update(&mut self, layout: &Layout, event: Event, shell: &mut Shell<Message>) -> event::Status {
        let mut state = self.state.0.borrow_mut();
        let focused = Some(state.id) == shell.focused();

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter | KeyCode::Char(' ') if focused => {
                    if let Some(message) = self.on_press.clone() {
                        shell.emit(message);
                        return event::Status::Captured;
                    }
                }
                _ => {}
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    let pos = Position::new(mouse.column, mouse.row);

                    if layout.area.contains(pos) && self.on_press.is_some() {
                        shell.request_redraw();
                        state.pressed = true;
                        return event::Status::Captured;
                    }
                }
                MouseEventKind::Up(MouseButton::Left) => {
                    state.pressed = false;

                    let pos = Position::new(mouse.column, mouse.row);

                    if layout.area.contains(pos) {
                        state.hovered = true;

                        if let Some(message) = self.on_press.clone() {
                            shell.emit(message);
                            return event::Status::Captured;
                        }
                    } else if focused {
                        shell.unfocus();
                        return event::Status::Captured;
                    }
                }
                MouseEventKind::Moved => {
                    let prev = state.hovered;
                    if mouse.kind == MouseEventKind::Moved {
                        let pos = Position::new(mouse.column, mouse.row);
                        state.hovered = layout.area.contains(pos);

                        if state.hovered != prev {
                            shell.request_redraw();
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }

        self.content.update(&layout.children[0], event, shell)
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout, focused: Option<widget::Id>) {
        let state = self.state.0.borrow();

        let status = if state.pressed {
            Status::Pressed
        } else if Some(state.id) == focused || state.hovered {
            Status::Hovered
        } else {
            Status::Inactive
        };
        let style = (self.style)(status);

        let border = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(style.borders);

        frame.render_widget(border, layout.area);

        self.content.render(frame, &layout.children[0], focused);
    }

    fn flatten(&self) -> Vec<widget::Info> {
        let state = &self.state.0.borrow();

        Some(widget::Info::focusable(state.id))
            .into_iter()
            .chain(self.content.flatten())
            .collect()
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: Button<'a, Message>) -> Self {
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
    Pressed,
}
