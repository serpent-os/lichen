// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use crossterm::event::KeyCode;
use lichen::{pages::users::Users, Action, Component, Event, Screen};

struct App<'a> {
    redraw: bool,
    quit: bool,
    page: Users<'a>,
}

impl<'a> Component for App<'a> {
    fn render(&self, frame: &mut ratatui::prelude::Frame) {
        self.page.render(frame)
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        self.page.update(action)
    }
}

impl<'a> App<'a> {
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
            Event::Mouse(m) => Some(Action::Mouse(m)),
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
        page: Users::new(),
    };

    loop {
        if app.redraw {
            screen.draw(|f| app.render(f))?;
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
