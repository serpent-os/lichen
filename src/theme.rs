// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Theme definitions

use ratatui::style::Color;

pub struct Theme {
    /// Text colour
    pub color_text: Color,

    /// Selection colour
    pub color_selection: Color,

    /// Highlight (hover) colour
    pub color_highlight: Color,

    /// Inactive (not-focused) colour
    pub color_inactive: Color,
}

pub static BASIC: Theme = Theme {
    color_text: Color::White,
    color_selection: Color::LightBlue,
    color_highlight: Color::White,
    color_inactive: Color::Gray,
};

/// Access the default theme
pub fn current() -> &'static Theme {
    &BASIC
}
