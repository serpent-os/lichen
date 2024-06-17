// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use crate::{widget, Element, Layout, Widget};

pub fn wrap<'a>(widget: impl ratatui::widgets::WidgetRef + 'a) -> Wrap<'a> {
    Wrap::new(widget)
}

pub struct Wrap<'a> {
    widget: Box<dyn ratatui::widgets::WidgetRef + 'a>,
}

impl<'a> Wrap<'a> {
    pub fn new(widget: impl ratatui::widgets::WidgetRef + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }
}

impl<'a, Message: 'a> Widget<Message> for Wrap<'a> {
    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout, _focused: Option<widget::Id>) {
        self.widget.render_ref(layout.area, frame.buffer_mut());
    }
}

impl<'a, Message> From<Wrap<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: Wrap<'a>) -> Self {
        Element::new(value)
    }
}
