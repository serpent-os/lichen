// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use std::mem;

use crossterm::event::KeyCode;
use lichen::tui::{
    application::{self, Command},
    event,
    widget::block,
    Application, Element, Event,
};
use ratatui::widgets::{Borders, Padding};

use self::page::Page;

mod page;
mod theme;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    application::run(App::new()).await
}

enum Message {
    Page(page::Message),
    Quit,
}

struct App {
    history: Vec<Page>,
    current: Page,
}

impl App {
    fn new() -> Self {
        App {
            history: vec![],
            current: Page::welcome(),
        }
    }
}

impl Application for App {
    type Message = Message;

    fn handle(&self, event: Event, status: event::Status) -> Option<Self::Message> {
        match event {
            Event::Key(e) if status == event::Status::Ignored => {
                if e.code == KeyCode::Char('q') {
                    return Some(Message::Quit);
                }
            }
            _ => {}
        }
        None
    }

    fn update(&mut self, message: Message) -> Option<Command<Message>> {
        match message {
            Message::Page(message) => {
                match self.current.update(message) {
                    Some(event) => match event {
                        page::Event::Welcome(event) => match event {
                            page::welcome::Event::Ok => {
                                self.history
                                    .push(mem::replace(&mut self.current, Page::user()));
                            }
                        },
                        page::Event::User(event) => match event {
                            page::user::Event::User { username, password } => {
                                println!("User submitted:\n  username: {username}\n  password: {password}");
                            }
                            page::user::Event::Cancel => {
                                if let Some(prev) = self.history.pop() {
                                    self.current = prev;
                                }
                            }
                        },
                    },
                    _ => {}
                }

                None
            }
            Message::Quit => Some(Command::Quit),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, Self::Message> {
        block(self.current.view().map(Message::Page))
            .padding(Padding::uniform(2))
            .borders(Borders::NONE)
            .into()
    }
}
