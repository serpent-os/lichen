// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Screen management

use std::{
    io::{self, stdout},
    ops::{Deref, DerefMut},
    panic,
};

use crossterm::{
    cursor, execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub struct Screen {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Screen {
    pub fn new() -> Result<Self, io::Error> {
        execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        enable_raw_mode()?;
        let term = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal: term })
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        end_tty().expect("Failed to end tty use")
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
        stdout().execute(LeaveAlternateScreen)?;
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
