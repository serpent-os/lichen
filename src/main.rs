// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! TUI frontend for lichen

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    lichen::install_eyre_hooks()?;

    Ok(())
}
