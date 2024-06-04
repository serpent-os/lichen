// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! BoxLayout ...

use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Direction, Flex, Layout, Rect};

use crate::{component::State, Action, Component};

pub struct BoxLayout {
    direction: Direction,
    children: RefCell<Vec<Rc<dyn Component>>>,
    selected: RefCell<usize>,
    flex: Flex,
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self {
            direction: Direction::Horizontal,
            children: RefCell::new(Vec::new()),
            selected: RefCell::new(0),
            flex: Flex::Legacy,
        }
    }
}

impl BoxLayout {
    pub fn new(children: Vec<Rc<dyn Component>>) -> Self {
        let s = Self {
            direction: Direction::Horizontal,
            children: RefCell::new(children),
            selected: RefCell::new(0),
            flex: Flex::Legacy,
        };
        s.update_states();
        s
    }

    pub fn flex(self, flex: Flex) -> Self {
        Self { flex, ..self }
    }

    pub fn push(&self, child: Rc<dyn Component>) {
        self.children.borrow_mut().push(child);
    }

    // Update the direction
    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    fn traverse_tab(&self) -> Option<Action> {
        let children = self.children.borrow_mut();
        let mut selected = self.selected.borrow_mut();
        if children.is_empty() {
            return None;
        }

        if *selected + 1 >= children.len() {
            *selected = 0;
        } else {
            *selected += 1;
        }

        drop(children);
        drop(selected);

        self.update_states();
        None
    }

    fn traverse_tab_r(&self) -> Option<Action> {
        let children = self.children.borrow();
        let mut selected = self.selected.borrow_mut();
        if children.is_empty() {
            return None;
        }

        if *selected == 0 {
            *selected = children.len() - 1;
        } else {
            *selected -= 1;
        }

        drop(children);
        drop(selected);
        self.update_states();
        None
    }

    fn update_states(&self) {
        let selected = self.selected.borrow();
        for (index, child) in self.children.borrow().iter().enumerate() {
            child.pop_state(State::ACTIVE);
            if index == *selected {
                child.push_state(State::ACTIVE);
            }
        }
    }
}

impl Component for BoxLayout {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        let children = self.children.borrow();
        let layout = match self.direction {
            Direction::Horizontal => {
                Layout::horizontal(children.iter().map(|c| c.constraints(self.direction)))
                    .flex(self.flex)
            }
            Direction::Vertical => {
                Layout::vertical(children.iter().map(|c| c.constraints(self.direction)))
                    .flex(self.flex)
            }
        }
        .spacing(1)
        .split(area);

        for (index, child) in children.iter().enumerate() {
            child.render(frame, layout[index]);
        }
    }

    fn update(&self, action: crate::Action) -> Option<crate::Action> {
        if let Action::Key(k) = action {
            if k.kind == KeyEventKind::Press {
                match k.code {
                    KeyCode::Tab | KeyCode::Down => return self.traverse_tab(),
                    KeyCode::BackTab | KeyCode::Up => return self.traverse_tab_r(),
                    _ => {}
                };
            }
        }

        let mut children = self.children.borrow_mut();
        let selected = *self.selected.borrow();

        if let Some(child) = children.get_mut(selected) {
            child.update(action)
        } else {
            None
        }
    }

    fn state(&self) -> State {
        State::NONE
    }

    fn push_state(&self, st: crate::component::State) {}

    fn pop_state(&self, st: crate::component::State) {}
}
