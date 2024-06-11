// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Trivial button widget

use crossterm::event::{KeyEventKind, MouseButton};
use ratatui::{
    layout::{Constraint, Position, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding},
};

use crate::tui::{event, layout, Element, Event, Layout, Shell, Widget};

pub fn button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    Button {
        content: content.into(),
        padding: Padding::new(1, 1, 0, 0),
        on_press: None,
        border_style: Style::default(),
    }
}

pub struct Button<'a, Message> {
    content: Element<'a, Message>,
    padding: Padding,
    on_press: Option<Message>,
    border_style: Style,
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

    pub fn border_style(self, style: impl Into<Style>) -> Self {
        Self {
            border_style: style.into(),
            ..self
        }
    }
}

impl<'a, Message> Widget<Message> for Button<'a, Message>
where
    Message: Clone + 'a,
{
    fn width(&self) -> Constraint {
        let left = self.padding.left + 1;
        let right = self.padding.right + 1;

        layout::pad_constraint(self.content.width(), left + right)
    }

    fn height(&self) -> Constraint {
        let top = self.padding.top + 1;
        let bottom = self.padding.bottom + 1;

        layout::pad_constraint(self.content.height(), top + bottom)
    }

    fn layout(&self, available: Rect) -> Layout {
        let mut padding = self.padding.clone();
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

    fn update(
        &mut self,
        layout: &Layout,
        event: Event,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        match event {
            Event::Key(key) => match key.code {
                // TODO: Focus
                crossterm::event::KeyCode::Enter if key.kind == KeyEventKind::Release => {
                    if let Some(message) = self.on_press.clone() {
                        shell.emit(message);
                        return event::Status::Captured;
                    }
                }
                _ => {}
            },
            Event::Mouse(mouse) => match mouse.kind {
                crossterm::event::MouseEventKind::Up(MouseButton::Left) => {
                    let pos = Position::new(mouse.column, mouse.row);

                    if layout.area.contains(pos) {
                        if let Some(message) = self.on_press.clone() {
                            shell.emit(message);
                            return event::Status::Captured;
                        }
                    }
                }
                _ => {}
            },
        }

        self.content.update(&layout.children[0], event, shell)
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout) {
        let border = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(self.border_style);

        frame.render_widget(border, layout.area);

        self.content.render(frame, &layout.children[0]);
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
