// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! BoxLayout provides a managed wrapper around the ratatui Layout types
//! to allow nested tree of "objects" (widgets)

use ratatui::layout::{self, Constraint, Direction, Flex, Rect};

use crate::tui::{event, Element, Event, Layout, Shell, Widget};

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
    fn width(&self) -> Constraint {
        // TODO: Configurable
        Constraint::Fill(1)
    }

    fn height(&self) -> Constraint {
        // TODO: Configurable
        Constraint::Fill(1)
    }

    fn layout(&self, available: Rect) -> Layout {
        let children = match self.direction {
            Direction::Horizontal => {
                layout::Layout::horizontal(self.children.iter().map(|child| child.width()))
            }
            Direction::Vertical => {
                layout::Layout::vertical(self.children.iter().map(|child| child.height()))
            }
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

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout) {
        self.children
            .iter()
            .zip(&layout.children)
            .for_each(|(child, layout)| child.render(frame, layout));
    }
    // /// Render only children, recursively, via root level layout
    // fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
    //     let children = self.children.borrow();
    //     let layout = match self.direction {
    //         Direction::Horizontal => {
    //             Layout::horizontal(children.iter().map(|c| c.constraints(self.direction)))
    //                 .flex(self.flex)
    //         }
    //         Direction::Vertical => {
    //             Layout::vertical(children.iter().map(|c| c.constraints(self.direction)))
    //                 .flex(self.flex)
    //         }
    //     }
    //     .spacing(1)
    //     .split(area);

    //     for (index, child) in children.iter().enumerate() {
    //         child.render(frame, layout[index]);
    //     }
    // }

    // /// Handle some keyboard shortcuts, or pass to children
    // fn update(&mut self, event: Event, shell: &mut Shell<Message>) -> event::Status {
    //     if let Event::Key(k) = event {
    //         if k.kind == KeyEventKind::Press {
    //             match k.code {
    //                 KeyCode::Tab | KeyCode::Down => return self.traverse_tab(),
    //                 KeyCode::BackTab | KeyCode::Up => return self.traverse_tab_r(),
    //                 _ => {}
    //             };
    //         }
    //     }

    //     let mut children = self.children.borrow_mut();
    //     let selected = *self.selected.borrow();

    //     if let Some(child) = children.get_mut(selected) {
    //         child.update(action)
    //     } else {
    //         None
    //     }
    // }

    // /// No state needed
    // fn state(&self) -> State {
    //     State::NONE
    // }

    // /// Ditto
    // fn push_state(&self, _: crate::widget::State) {}

    // /// Ditto
    // fn pop_state(&self, _: crate::widget::State) {}
}

impl<'a, Message> From<BoxLayout<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: BoxLayout<'a, Message>) -> Self {
        Element::new(value)
    }
}
