// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Screen management

use std::{
    io::{self, stdout},
    panic,
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// Initialise terminal integrations
pub fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;
    Ok(term)
}

/// Finish terminal integrations
pub fn finish() -> Result<(), io::Error> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Properly handle eyre hooks by resetting the display first
pub fn install_eyre_hooks() -> color_eyre::Result<()> {
    let builder = color_eyre::config::HookBuilder::default();
    let (p, e) = builder.into_hooks();
    let p = p.into_panic_hook();
    panic::set_hook(Box::new(move |info| {
        finish().unwrap();
        p(info);
    }));

    let e = e.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |err| {
        finish().unwrap();
        e(err)
    }))?;

    Ok(())
}
