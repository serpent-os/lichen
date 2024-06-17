// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Theme definitions

use std::env;
use std::sync::OnceLock;

use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Style, Stylize};
use tui::widget::{self, text_box};

/// Add terminals to this list that don't report TERM=xterm-256color but
/// actually do support full features..
static SILLY_BUGGERS: [&str; 1] = ["alacritty"];

/// Map well known icon names for each environment
pub struct Icons {
    // Represent a username
    pub user: &'static str,

    /// Represent a password.
    pub password: &'static str,
}

/// Provides a simple means to override the palette per environment
pub struct Theme {
    /// Selection colour
    pub color_selection: Color,

    /// Highlight (hover) colour
    pub color_highlight: Color,

    /// Inactive (not-focused) colour
    pub color_inactive: Color,

    /// Icon set
    pub icons: Icons,
}

/// Basic theme for tty/non-256/emoji use
pub static BASIC: Theme = Theme {
    color_selection: Color::LightBlue,
    color_highlight: Color::White,
    color_inactive: Color::DarkGray,
    icons: Icons {
        user: " + ",
        password: " # ",
    },
};

/// Refined theme for desktop use
pub static REFINED: Theme = Theme {
    color_selection: tailwind::BLUE.c300,
    color_highlight: tailwind::SLATE.c400,
    color_inactive: tailwind::SLATE.c500,
    icons: Icons {
        user: " 👤 ",
        password: " 🔑 ",
    },
};

/// Access the default theme
pub fn current() -> &'static Theme {
    static RES: OnceLock<&'static Theme> = OnceLock::new();
    RES.get_or_init(|| match crossterm::style::available_color_count() {
        x if x > 255 => &REFINED,
        _ => {
            let term = env::var("TERM").unwrap_or_default();
            if SILLY_BUGGERS.iter().any(|s| *s == term) {
                &REFINED
            } else {
                &BASIC
            }
        }
    })
}

pub fn text_box(status: text_box::Status, masked: bool) -> text_box::Stylesheet {
    let theme = current();

    let bold_mask = |style: Style| if masked { style.bold() } else { style };

    match status {
        text_box::Status::Inactive => text_box::Stylesheet {
            area: bold_mask(Style::default().fg(theme.color_inactive)),
            cursor: Style::default(),
            borders: theme.color_inactive.into(),
        },
        text_box::Status::Hovered => text_box::Stylesheet {
            area: bold_mask(Style::default().fg(theme.color_inactive)),
            cursor: Style::default(),
            borders: theme.color_highlight.into(),
        },
        text_box::Status::Active => text_box::Stylesheet {
            area: bold_mask(Style::default()),
            cursor: Style::default().reversed(),
            borders: theme.color_selection.into(),
        },
    }
}

pub fn button(status: widget::button::Status) -> widget::button::Stylesheet {
    let theme = current();

    match status {
        widget::button::Status::Inactive => widget::button::Stylesheet {
            borders: theme.color_inactive.into(),
        },
        widget::button::Status::Hovered => widget::button::Stylesheet {
            borders: theme.color_highlight.into(),
        },
        widget::button::Status::Pressed => widget::button::Stylesheet {
            borders: theme.color_selection.into(),
        },
    }
}
