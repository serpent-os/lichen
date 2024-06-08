// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! BoxLayout provides a managed wrapper around the ratatui Layout types
//! to allow nested tree of "objects" (widgets)

use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Direction, Flex, Layout, Rect};

use crate::{widget::State, Action, Widget};

/// BoxLayout type
pub struct BoxLayout {
    direction: Direction,
    children: RefCell<Vec<Rc<dyn Widget>>>,
    selected: RefCell<usize>,
    flex: Flex,
}

impl BoxLayout {
    /// Create a new BoxLayout
    ///
    /// # Arguments
    ///
    ///  - `children` - Child widgets
    pub fn new(children: Vec<Rc<dyn Widget>>) -> Self {
        let children = RefCell::new(children);
        let selected = RefCell::new(0);
        let s = Self {
            direction: Direction::Horizontal,
            children,
            selected,
            flex: Flex::Legacy,
        };
        s.update_states(s.children.borrow(), s.selected.borrow_mut());
        s
    }

    /// Set the flex property
    pub fn flex(self, flex: Flex) -> Self {
        Self { flex, ..self }
    }

    // Update the direction
    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    /// Handle tab traversal (focus policy)
    fn traverse_tab(&self) -> Option<Action> {
        let children = self.children.borrow();
        let mut selected = self.selected.borrow_mut();
        if children.is_empty() {
            return None;
        }

        if *selected + 1 >= children.len() {
            *selected = 0;
        } else {
            *selected += 1;
        }

        self.update_states(children, selected);
        None
    }

    /// Handle shift+tab (reverse tab traversal)
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

        self.update_states(children, selected);
        None
    }

    /// Update all states within this container due to selections
    fn update_states(&self, children: Ref<'_, Vec<Rc<dyn Widget>>>, selected: RefMut<'_, usize>) {
        for (index, child) in children.borrow().iter().enumerate() {
            child.pop_state(State::ACTIVE);
            if index == *selected {
                child.push_state(State::ACTIVE);
            }
        }
    }
}

impl Widget for BoxLayout {
    /// Render only children, recursively, via root level layout
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

    /// Handle some keyboard shortcuts, or pass to children
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

    /// No state needed
    fn state(&self) -> State {
        State::NONE
    }

    /// Ditto
    fn push_state(&self, _: crate::widget::State) {}

    /// Ditto
    fn pop_state(&self, _: crate::widget::State) {}
}
