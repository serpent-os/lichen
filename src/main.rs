// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use crossterm::event::KeyCode;
use lichen::{Action, Component, Event, Screen};
use ratatui::widgets::{self};

struct App {
    redraw: bool,
    quit: bool,
}

impl Component for App {
    fn widget(&self) -> impl ratatui::prelude::Widget {
        widgets::Paragraph::new("Welcome to Lichen")
    }

    fn update(&self, _: Action) -> Option<Action> {
        None
    }
}

impl App {
    fn handle(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(e) => {
                if e.code == KeyCode::Char('q') {
                    self.quit = true;
                    Some(Action::Quit)
                } else {
                    Some(Action::Key(e))
                }
            }
            Event::Render => {
                self.redraw = true;
                None
            }
            _ => None,
        }
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    lichen::install_eyre_hooks()?;

    let mut screen = Screen::new()?;
    screen.run();

    let mut app = App {
        redraw: false,
        quit: false,
    };

    loop {
        if app.redraw {
            screen.draw(|f| {
                let area = f.size();
                f.render_widget(app.widget(), area)
            })?;
            app.redraw = false;
        }

        if let Some(event) = screen.next_event().await {
            let mut act = app.handle(event);
            while let Some(action) = act {
                act = app.update(action);
            }
        }

        if app.quit {
            break;
        }
    }
    screen.stop();
    Ok(())
}
