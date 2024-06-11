// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use crossterm::event::KeyCode;
use lichen::tui::{
    application::{self, Command},
    event,
    widget::block,
    Application, Element, Event, Widget,
};
use pages::user;
use ratatui::widgets::{Borders, Padding};

use self::pages::User;

mod pages;
mod theme;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    application::run(App {
        user: User::default(),
    })
    .await?;
    Ok(())
}

enum Message {
    User(user::Message),
    Quit,
}

struct App {
    user: User,
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
            Message::User(message) => {
                match self.user.update(message) {
                    Some(event) => {
                        match event {
                            user::Event::User { username, password } => {
                                println!("User submitted:\n  username: {username}\n  password: {password}");
                            }
                        }
                    }
                    None => {}
                }

                None
            }
            Message::Quit => Some(Command::Quit),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, Self::Message> {
        block(self.user.view().map(Message::User))
            .padding(Padding::uniform(2))
            .borders(Borders::NONE)
            .into()
    }
}
