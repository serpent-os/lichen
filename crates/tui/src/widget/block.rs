// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{BorderType, Borders, Padding},
};

use crate::{event, layout, widget, Element, Event, Layout, Shell, Widget};

pub fn block<'a, Message>(content: impl Into<Element<'a, Message>>) -> Block<'a, Message> {
    Block::new(content)
}

pub struct Block<'a, Message> {
    content: Element<'a, Message>,
    padding: Padding,
    title: Option<String>,
    borders: Borders,
    border_style: Style,
}

impl<'a, Message> Block<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            padding: Padding::new(1, 1, 0, 0),
            title: None,
            borders: Borders::ALL,
            border_style: Style::default(),
        }
    }

    pub fn padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    pub fn title(self, title: impl ToString) -> Self {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }

    pub fn borders(self, borders: Borders) -> Self {
        Self { borders, ..self }
    }

    pub fn border_style(self, style: impl Into<Style>) -> Self {
        Self {
            border_style: style.into(),
            ..self
        }
    }
}

impl<'a, Message: 'a> Widget<Message> for Block<'a, Message> {
    fn width(&self, height: u16) -> Constraint {
        let left = self.borders.contains(Borders::LEFT).then_some(1).unwrap_or_default() + self.padding.left;
        let right = self.borders.contains(Borders::RIGHT).then_some(1).unwrap_or_default() + self.padding.right;

        layout::pad_constraint(self.content.width(height), left + right)
    }

    fn height(&self, width: u16) -> Constraint {
        let top = self.borders.contains(Borders::TOP).then_some(1).unwrap_or_default() + self.padding.top;
        let bottom = self.borders.contains(Borders::BOTTOM).then_some(1).unwrap_or_default() + self.padding.bottom;

        layout::pad_constraint(self.content.height(width), top + bottom)
    }

    fn layout(&self, available: Rect) -> Layout {
        let left = self.borders.contains(Borders::LEFT).then_some(1).unwrap_or_default();
        let right = self.borders.contains(Borders::RIGHT).then_some(1).unwrap_or_default();
        let top = self.borders.contains(Borders::TOP).then_some(1).unwrap_or_default();
        let bottom = self.borders.contains(Borders::BOTTOM).then_some(1).unwrap_or_default();

        let mut padding = self.padding;
        padding.left += left;
        padding.right += right;
        padding.top += top;
        padding.bottom += bottom;

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
        self.content.update(&layout.children[0], event, shell)
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout, focused: Option<widget::Id>) {
        let mut border = ratatui::widgets::Block::default()
            .border_type(BorderType::Rounded)
            .border_style(self.border_style)
            .borders(self.borders);

        if let Some(title) = &self.title {
            border = border.title(title.as_str());
        }

        frame.render_widget(border, layout.area);

        self.content.render(frame, &layout.children[0], focused);
    }

    fn flatten(&self) -> Vec<widget::Info> {
        Some(widget::Info::default())
            .into_iter()
            .chain(self.content.flatten())
            .collect()
    }
}

impl<'a, Message> From<Block<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: Block<'a, Message>) -> Self {
        Element::new(value)
    }
}
