// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

use tui::screen;

fn main() -> color_eyre::Result<()> {
    screen::install_eyre_hooks()?;

    let _ = screen::init()?;
    todo!()
}
