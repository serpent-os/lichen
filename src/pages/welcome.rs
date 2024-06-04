// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Welcome page

use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{self, Wrap},
};

use crate::Component;

pub struct Welcome {}

impl Default for Welcome {
    fn default() -> Self {
        Self::new()
    }
}

impl Welcome {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Welcome {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let intro = widgets::Paragraph::new(vec![
            Line::from("Are you ready to install Serpent OS?").style(Style::default().bold()).alignment(ratatui::layout::Alignment::Center),
            Line::from(""), Line::from(""),
            Line::from("📋 Disclaimer").style(Style::default().bold()),
            Line::from(""),
            Line::from("Serpent OS is not ready for production use nor should it be used for market predictions, nuclear facility management or the care of infants. At any point your installation may encounter errors or in fact no longer have ever been installed due to temporal fluctations resulting from the misuse of this install in a particle accelerator.").style(Style::default().dim())
        ]).wrap(Wrap{ trim: false });

        frame.render_widget(intro, area)
    }

    fn update(&self, _: crate::Action) -> Option<crate::Action> {
        None
    }

    fn state(&self) -> crate::State {
        crate::State::NONE
    }

    fn push_state(&self, _: crate::State) {}

    fn pop_state(&self, _: crate::State) {}
}
