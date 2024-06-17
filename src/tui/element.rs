// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use ratatui::{
    layout::{Constraint, Rect},
    Frame,
};

use crate::tui::{event, Event, Layout, Shell, Widget};

use super::widget;

pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message: 'a> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn width(&self, height: u16) -> Constraint {
        self.widget.width(height)
    }

    pub fn height(&self, width: u16) -> Constraint {
        self.widget.height(width)
    }

    pub fn layout(&self, available: Rect) -> Layout {
        self.widget.layout(available)
    }

    pub fn update(&mut self, layout: &Layout, event: Event, shell: &mut Shell<Message>) -> event::Status {
        self.widget.update(layout, event, shell)
    }

    pub fn render(&self, frame: &mut Frame, layout: &Layout, focused: Option<widget::Id>) {
        self.widget.render(frame, layout, focused)
    }

    pub fn flatten(&self) -> Vec<widget::Info> {
        self.widget.flatten()
    }

    pub fn map<U: 'a>(self, f: impl Fn(Message) -> U + 'a) -> Element<'a, U> {
        struct Map<'a, T, U> {
            widget: Box<dyn Widget<T> + 'a>,
            mapper: Box<dyn Fn(T) -> U + 'a>,
        }

        impl<'a, T, U> Widget<U> for Map<'a, T, U> {
            fn width(&self, height: u16) -> Constraint {
                self.widget.width(height)
            }

            fn height(&self, width: u16) -> Constraint {
                self.widget.height(width)
            }

            fn layout(&self, available: Rect) -> Layout {
                self.widget.layout(available)
            }

            fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout, focused: Option<widget::Id>) {
                self.widget.render(frame, layout, focused)
            }

            fn update(&mut self, layout: &Layout, event: Event, shell: &mut Shell<U>) -> event::Status {
                let mut local_shell = Shell::with_focused(shell.focused());

                let status = self.widget.update(layout, event, &mut local_shell);

                shell.merge(local_shell.map(&self.mapper));

                status
            }

            fn flatten(&self) -> Vec<widget::Info> {
                self.widget.flatten()
            }
        }

        Element {
            widget: Box::new(Map {
                widget: self.widget,
                mapper: Box::new(f),
            }),
        }
    }
}
