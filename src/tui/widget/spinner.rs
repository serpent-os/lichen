// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

use ratatui::layout::Constraint;

use crate::tui::{event, widget, Element, Event, Layout, Shell, Widget};

const UPDATE_INTERVAL: Duration = Duration::from_millis(150);

pub fn spinner(state: &State, chars: Vec<char>) -> Spinner<'_> {
    Spinner::new(state, chars)
}

#[derive(Default)]
pub struct State(RefCell<Inner>);

#[derive(Default)]
struct Inner {
    current: usize,
    last_redraw: Option<Instant>,
}

pub struct Spinner<'a> {
    state: &'a State,
    chars: Vec<char>,
}

impl<'a> Spinner<'a> {
    pub fn new(state: &'a State, chars: Vec<char>) -> Self {
        Self { state, chars }
    }
}

impl<'a, Message: 'a> Widget<Message> for Spinner<'a> {
    fn width(&self, _height: u16) -> Constraint {
        Constraint::Length(1)
    }

    fn height(&self, _width: u16) -> Constraint {
        Constraint::Length(1)
    }

    fn update(&mut self, _layout: &Layout, event: Event, shell: &mut Shell<Message>) -> event::Status {
        let mut state = self.state.0.borrow_mut();

        if let Event::RedrawRequested(now) = event {
            match state.last_redraw {
                Some(last_redraw) => {
                    if now.duration_since(last_redraw) >= UPDATE_INTERVAL {
                        state.current = (state.current + 1) % self.chars.len();
                        shell.request_redraw_after(UPDATE_INTERVAL);
                    }
                }
                None => {
                    shell.request_redraw_after(UPDATE_INTERVAL);
                }
            }

            state.last_redraw = Some(now);
        }

        event::Status::Ignored
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout, _focused: Option<widget::Id>) {
        let state = self.state.0.borrow();

        if let Some(c) = self.chars.get(state.current) {
            frame.render_widget(ratatui::text::Text::raw(format!("{c}")), layout.area)
        }
    }
}

impl<'a, Message> From<Spinner<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: Spinner<'a>) -> Self {
        Element::new(value)
    }
}
