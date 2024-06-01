// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! BoxLayout ...

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};

use crate::{Action, Component};

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct BoxLayout {
    orientation: Orientation,
    children: Vec<Box<dyn Component>>,
    selected: usize,
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            children: Vec::new(),
            selected: 0,
        }
    }
}

impl BoxLayout {
    pub fn new(children: Vec<Box<dyn Component>>) -> Self {
        Self {
            orientation: Orientation::Horizontal,
            children,
            selected: 0,
        }
    }

    pub fn push(&mut self, child: Box<dyn Component>) {
        self.children.push(child)
    }

    pub fn with_children(self, children: Vec<Box<dyn Component>>) -> Self {
        Self { children, ..self }
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
        None
    }
}

impl Component for BoxLayout {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        let layout = match self.orientation {
            Orientation::Horizontal => {
                Layout::horizontal(self.children.iter().map(|_| Constraint::Length(10)))
            }
            Orientation::Vertical => {
                Layout::vertical(self.children.iter().map(|_| Constraint::Length(3)))
            }
        }
        .split(area);

        for (index, child) in self.children.iter().enumerate() {
            child.render(frame, layout[index]);
        }
    }

    fn update(&mut self, action: crate::Action) -> Option<crate::Action> {
        if let Action::Key(k) = action {
            if k.code == KeyCode::Tab && k.kind == KeyEventKind::Press {
                if k.modifiers.contains(KeyModifiers::SHIFT) {
                    return self.traverse_tab_r();
                } else {
                    return self.traverse_tab();
                }
            }
        }
        if let Some(child) = self.children.get_mut(self.selected) {
            child.update(action)
        } else {
            None
        }
    }
}
