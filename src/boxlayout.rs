// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! BoxLayout ...

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Flex, Layout, Rect};

use crate::{
    component::{Orientation, State},
    Action, Component,
};

pub struct BoxLayout {
    orientation: Orientation,
    children: Vec<Box<dyn Component>>,
    selected: usize,
    flex: Flex,
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            children: Vec::new(),
            selected: 0,
            flex: Flex::Legacy,
        }
    }
}

impl BoxLayout {
    pub fn new(children: Vec<Box<dyn Component>>) -> Self {
        let mut s = Self {
            orientation: Orientation::Horizontal,
            children,
            selected: 0,
            flex: Flex::Legacy,
        };
        s.update_states();
        s
    }

    pub fn flex(self, flex: Flex) -> Self {
        Self { flex, ..self }
    }

    pub fn push(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }

    // Update the orientation
    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            orientation,
            ..self
        }
    }

    fn traverse_tab(&mut self) -> Option<Action> {
        if self.children.is_empty() {
            return None;
        }

        if self.selected + 1 >= self.children.len() {
            self.selected = 0;
        } else {
            self.selected += 1;
        }

        self.update_states();
        None
    }

    fn traverse_tab_r(&mut self) -> Option<Action> {
        if self.children.is_empty() {
            return None;
        }

        if self.selected == 0 {
            self.selected = self.children.len() - 1;
        } else {
            self.selected -= 1;
        }
        self.update_states();
        None
    }

    fn update_states(&mut self) {
        for (index, child) in self.children.iter_mut().enumerate() {
            child.pop_state(State::ACTIVE);
            if index == self.selected {
                child.push_state(State::ACTIVE);
            }
        }
    }
}

impl Component for BoxLayout {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        let layout = match self.orientation {
            Orientation::Horizontal => Layout::horizontal(
                self.children
                    .iter()
                    .map(|c| c.constraints(self.orientation)),
            )
            .flex(self.flex),
            Orientation::Vertical => Layout::vertical(
                self.children
                    .iter()
                    .map(|c| c.constraints(self.orientation)),
            )
            .flex(self.flex),
        }
        .spacing(1)
        .split(area);

        for (index, child) in self.children.iter().enumerate() {
            child.render(frame, layout[index]);
        }
    }

    fn update(&mut self, action: crate::Action) -> Option<crate::Action> {
        if let Action::Key(k) = action {
            if k.kind == KeyEventKind::Press {
                match k.code {
                    KeyCode::Tab => return self.traverse_tab(),
                    KeyCode::BackTab => return self.traverse_tab_r(),
                    _ => {}
                };
            }
        }

        if let Some(child) = self.children.get_mut(self.selected) {
            child.update(action)
        } else {
            None
        }
    }

    fn state(&self) -> State {
        State::NONE
    }

    fn push_state(&mut self, st: crate::component::State) {}

    fn pop_state(&mut self, st: crate::component::State) {}
}
