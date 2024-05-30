// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! GUI frontend for lichen

use cosmic::{
    app::{self, Settings},
    iced::Size,
};
use gui::app::LichenApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .antialiasing(true)
        .debug(false)
        .client_decorations(true)
        .size(Size::new(1024.0, 768.0));

    app::run::<LichenApp>(settings, ())?;

    Ok(())
}
