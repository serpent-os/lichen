// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Widget APIs

use bitflags::bitflags;
use ratatui::{
    layout::{Constraint, Rect},
    Frame,
};

use crate::tui::{event, Event, Layout, Shell};

pub use self::block::block;
pub use self::box_layout::{hbox, vbox};
pub use self::button::button;
pub use self::paragraph::paragraph;
pub use self::text::text;
pub use self::wrap::wrap;

pub mod block;
pub mod box_layout;
pub mod button;
pub mod paragraph;
pub mod text;
pub mod text_box;
pub mod wrap;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Attributes : u8 {
        const NONE = 1;
        const FOCUSABLE = 1 << 2;
    }
}

pub trait Widget<Message> {
    fn width(&self, _height: u16) -> Constraint {
        Constraint::Fill(1)
    }

    fn height(&self, _width: u16) -> Constraint {
        Constraint::Fill(1)
    }

    fn layout(&self, available: Rect) -> Layout {
        Layout {
            area: available,
            children: vec![],
        }
    }

    /// Pass an update down the chain of the widget
    fn update(
        &mut self,
        _layout: &Layout,
        _event: Event,
        _shell: &mut Shell<Message>,
    ) -> event::Status {
        event::Status::Ignored
    }

    /// Draw using the final ratatui Frame within the bounds of our layout
    ///
    /// # Arguments:
    ///
    /// - `frame` - Ratatui frame target
    /// - `layout` - Layout of our widget
    fn render(&self, frame: &mut Frame, layout: &Layout);
}
