// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Screen management
//! Heavily inspired by https://ratatui.rs/recipes/apps/terminal-and-event-handler/

use std::{
    io::{self, stdout},
    ops::{Deref, DerefMut},
    panic, thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture, KeyEvent, MouseEvent},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use futures::{FutureExt, StreamExt};
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub struct Screen {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,

    // Event processing
    events_in: UnboundedReceiver<Event>,
    events_out: UnboundedSender<Event>,
    task: JoinHandle<()>,
    cancel: CancellationToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

impl Screen {
    /// Create screen management and init the screen.
    pub fn new() -> Result<Self, io::Error> {
        // Required colour output due to input selections etc
        crossterm::style::force_color_output(true);

        execute!(
            stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;
        enable_raw_mode()?;
        let term = Terminal::new(CrosstermBackend::new(stdout()))?;

        let (events_out, events_in) = unbounded_channel::<Event>();
        let task = tokio::spawn(async {});
        let cancel = CancellationToken::new();
        Ok(Self {
            terminal: term,
            events_in,
            events_out,
            task,
            cancel,
        })
    }

    /// Stop screen management immediately
    pub fn stop(&self) {
        self.cancel.cancel();
        let mut tries = 0;
        while !self.task.is_finished() {
            thread::sleep(Duration::from_millis(1));
            if tries > 100 {
                self.task.abort();
            }
            if tries > 500 {
                log::error!("Failed to abort render task");
                break;
            }
            tries += 1;
        }
        end_tty().unwrap();
    }

    // Run the screen handling loop (events, etc)
    pub fn run(&mut self) {
        self.cancel.cancel();
        self.cancel = CancellationToken::new();

        let cancel = self.cancel.clone();
        let events_out = self.events_out.clone();

        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut renders = tokio::time::interval(Duration::from_secs_f64(1.0 / 30.0));
            let mut ticker = tokio::time::interval(Duration::from_secs_f64(1.0 / 4.0));
            events_out.send(Event::Init).unwrap();

            loop {
                let render = renders.tick();
                let input = ticker.tick();
                let cterm = reader.next().fuse();

                tokio::select! {
                    event = cterm => {
                        match event {
                            Some(Ok(event)) => {
                                log::trace!("Got an event: {event:?}");
                                match event {
                                    crossterm::event::Event::FocusGained => {},
                                    crossterm::event::Event::FocusLost => {},
                                    crossterm::event::Event::Key(key) => events_out.send(Event::Key(key)).unwrap(),
                                    crossterm::event::Event::Mouse(m) => events_out.send(Event::Mouse(m)).unwrap(),
                                    crossterm::event::Event::Paste(_) => {},
                                    crossterm::event::Event::Resize(_, _) => {},
                                }
                            },
                            Some(Err(err)) => {
                                log::error!("Got an error: {err}");
                            },
                            None => {},
                        }
                    },
                    // render timeout
                    _ = cancel.cancelled() => {
                        break;
                    }
                    _ = input => {
                        events_out.send(Event::Tick).unwrap();
                    }
                    _ = render => {
                        events_out.send(Event::Render).unwrap();
                    }
                }
            }
        });
    }

    /// Yield the next possible event
    pub async fn next_event(&mut self) -> Option<Event> {
        self.events_in.recv().await
    }
}

impl Deref for Screen {
    type Target = Terminal<CrosstermBackend<io::Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

/// Finish terminal integrations
fn end_tty() -> Result<(), io::Error> {
    if is_raw_mode_enabled()? {
        execute!(
            stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        disable_raw_mode()?;
    }
    Ok(())
}

/// Properly handle eyre hooks by resetting the display first
pub fn install_eyre_hooks() -> color_eyre::Result<()> {
    let builder = color_eyre::config::HookBuilder::default();
    let (p, e) = builder.into_hooks();
    let p = p.into_panic_hook();
    panic::set_hook(Box::new(move |info| {
        end_tty().unwrap();
        p(info);
    }));

    let e = e.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |err| {
        end_tty().unwrap();
        e(err)
    }))?;

    Ok(())
}
