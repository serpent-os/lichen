// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use ratatui::{widgets::Paragraph, Frame};
use tui::Screen;

fn draw_ui(frame: &mut Frame) {
    let area = frame.size();
    let widget = Paragraph::new("I is a paragraph");
    frame.render_widget(widget, area)
}

fn main() -> color_eyre::Result<()> {
    tui::install_eyre_hooks()?;

    let mut screen = Screen::new()?;
    screen.clear()?;

    loop {
        screen.draw(draw_ui)?;
        break;
    }
    todo!()
}
