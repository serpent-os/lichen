// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use lichen::{Event, Screen};
use ratatui::{
    widgets::{Block, Paragraph},
    Frame,
};

/// Test drawing
fn draw_ui(frame: &mut Frame) {
    let area = frame.size();
    let widget = Paragraph::new("Welcome to Lichen").block(Block::bordered());
    frame.render_widget(widget, area)
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    lichen::install_eyre_hooks()?;

    let mut screen = Screen::new()?;
    screen.run();

    let mut redraw = true;
    let mut quit = false;

    loop {
        if redraw {
            screen.draw(draw_ui)?;
            redraw = false;
        }

        if let Some(event) = screen.next_event().await {
            match event {
                Event::Key(_) => {
                    quit = true;
                }
                Event::Render => {
                    redraw = true;
                }
                _ => {}
            }
        }

        if quit {
            break;
        }
    }
    screen.stop();
    Ok(())
}
