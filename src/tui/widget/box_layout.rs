// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! BoxLayout provides a managed wrapper around the ratatui Layout types
//! to allow nested tree of "objects" (widgets)

use ratatui::layout::{self, Constraint, Direction, Flex, Rect};

use crate::tui::{event, widget, Element, Event, Layout, Shell, Widget};

pub fn hbox<'a, Message>(children: Vec<Element<'a, Message>>) -> BoxLayout<'a, Message> {
    BoxLayout::new(children).direction(Direction::Horizontal)
}

pub fn vbox<'a, Message>(children: Vec<Element<'a, Message>>) -> BoxLayout<'a, Message> {
    BoxLayout::new(children).direction(Direction::Vertical)
}

/// BoxLayout type
pub struct BoxLayout<'a, Message> {
    direction: Direction,
    children: Vec<Element<'a, Message>>,
    flex: Flex,
    spacing: u16,
}

impl<'a, Message> BoxLayout<'a, Message> {
    /// Create a new BoxLayout
    ///
    /// # Arguments
    ///
    ///  - `children` - Child widgets
    pub fn new(children: Vec<Element<'a, Message>>) -> Self {
        // let selected = RefCell::new(0);
        let s = Self {
            direction: Direction::Horizontal,
            children,
            flex: Flex::Legacy,
            spacing: 0,
        };
        // s.update_states(s.children.borrow(), s.selected.borrow_mut());
        s
    }

    /// Set the flex property
    pub fn flex(self, flex: Flex) -> Self {
        Self { flex, ..self }
    }

    pub fn spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    // Update the direction
    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }
}

impl<'a, Message: 'a> Widget<Message> for BoxLayout<'a, Message> {
    fn width(&self, _height: u16) -> Constraint {
        // TODO: Configurable
        Constraint::Fill(1)
    }

    fn height(&self, _width: u16) -> Constraint {
        // TODO: Configurable
        Constraint::Fill(1)
    }

    fn layout(&self, available: Rect) -> Layout {
        let children = match self.direction {
            Direction::Horizontal => layout::Layout::horizontal(
                self.children
                    .iter()
                    .map(|child| child.width(available.height)),
            ),
            Direction::Vertical => layout::Layout::vertical(
                self.children
                    .iter()
                    .map(|child| child.height(available.width)),
            ),
        }
        .flex(self.flex)
        .spacing(self.spacing)
        .split(available)
        .into_iter()
        .zip(&self.children)
        .map(|(area, child)| child.layout(*area))
        .collect();

        Layout {
            area: available,
            children,
        }
    }

    fn update(
        &mut self,
        layout: &Layout,
        event: Event,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        self.children
            .iter_mut()
            .zip(&layout.children)
            .map(|(child, layout)| child.update(layout, event.clone(), shell))
            .max()
            .unwrap_or_default()
    }

    fn render(
        &self,
        frame: &mut ratatui::prelude::Frame,
        layout: &Layout,
        focused: Option<widget::Id>,
    ) {
        self.children
            .iter()
            .zip(&layout.children)
            .for_each(|(child, layout)| child.render(frame, layout, focused));
    }

    fn flatten(&self) -> Vec<widget::Info> {
        Some(widget::Info::default())
            .into_iter()
            .chain(self.children.iter().flat_map(|child| child.flatten()))
            .collect()
    }
}

impl<'a, Message> From<BoxLayout<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: BoxLayout<'a, Message>) -> Self {
        Element::new(value)
    }
}
