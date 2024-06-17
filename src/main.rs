// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use std::{env, mem};

use crossterm::event::{KeyCode, KeyEventKind};
use lichen::tui::{
    application::{self, Command},
    event,
    widget::block,
    Application, Element, Event,
};
use ratatui::widgets::{Borders, Padding};
use system::{disk::Disk, locale};

use self::page::Page;

mod page;
mod system;
mod theme;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // find all disks
    let disks = Disk::discover()?;
    eprintln!("System disks: {disks:?}");
    let lang = env::var("LANG").unwrap_or_default();
    let registry = locale::Registry::new()?;
    if let Some(locale) = registry.locale(lang) {
        eprintln!("Found your current locale: {locale:?}");
    } else {
        eprintln!("Couldn't find your current locale. Is LANG set?");
    }
    application::run(App::new()).await
}

enum Message {
    Page(page::Message),
    Quit,
    FocusNext,
    FocusPrevious,
    Unfocus,
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
            Event::Key(e) if status == event::Status::Ignored && e.kind == KeyEventKind::Press => match e.code {
                KeyCode::Char('q') => Some(Message::Quit),
                KeyCode::Tab => Some(Message::FocusNext),
                KeyCode::BackTab => Some(Message::FocusPrevious),
                KeyCode::Esc => Some(Message::Unfocus),
                _ => None,
            },
            _ => None,
        }
    }

    fn update(&mut self, message: Message) -> Option<Command<Message>> {
        match message {
            Message::Page(message) => {
                if let Some(event) = self.current.update(message) {
                    match event {
                        page::Event::Welcome(event) => match event {
                            page::welcome::Event::Ok => {
                                self.history.push(mem::replace(&mut self.current, Page::user()));
                                return Some(Command::focus_next());
                            }
                        },
                        page::Event::User(event) => match event {
                            page::user::Event::User { username, password } => {
                                println!("User submitted:\n  username: {username}\n  password: {password}");
                                self.history.push(mem::replace(&mut self.current, Page::timezone()));
                                return Some(Command::focus_next());
                            }
                            page::user::Event::Cancel => {
                                if let Some(prev) = self.history.pop() {
                                    self.current = prev;
                                }
                            }
                        },
                        page::Event::Timezone(event) => match event {
                            page::timezone::Event::Timezone(tz) => {
                                println!("Timezone: {tz}");
                            }
                            page::timezone::Event::Cancel => {
                                if let Some(prev) = self.history.pop() {
                                    self.current = prev;
                                }
                            }
                        },
                    };
                }

                None
            }
            Message::Quit => Some(Command::Quit),
            Message::FocusNext => Some(Command::focus_next()),
            Message::FocusPrevious => Some(Command::focus_previous()),
            Message::Unfocus => Some(Command::unfocus()),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        block(self.current.view().map(Message::Page))
            .padding(Padding::uniform(2))
            .borders(Borders::NONE)
            .into()
    }
}
