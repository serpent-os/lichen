// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use ratatui::{
    layout::Constraint,
    text::Line,
    widgets::{WidgetRef, Wrap},
};

use crate::tui::{widget, Element, Layout, Widget};

pub fn paragraph(lines: Vec<Line<'_>>) -> Paragraph<'_> {
    Paragraph::new(lines)
}

pub struct Paragraph<'a> {
    inner: ratatui::widgets::Paragraph<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Self {
            inner: ratatui::widgets::Paragraph::new(lines),
        }
    }

    pub fn wrap(self) -> Self {
        Self {
            inner: self.inner.wrap(Wrap { trim: false }),
        }
    }
}

impl<'a, Message: 'a> Widget<Message> for Paragraph<'a> {
    fn height(&self, width: u16) -> Constraint {
        Constraint::Length(self.inner.line_count(width) as u16)
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout, _focused: Option<widget::Id>) {
        self.inner.render_ref(layout.area, frame.buffer_mut());
    }
}

impl<'a, Message> From<Paragraph<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: Paragraph<'a>) -> Self {
        Element::new(value)
    }
}
