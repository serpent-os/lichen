// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

mod screen;

pub use screen::install_eyre_hooks;
pub use screen::Event;
pub use screen::Screen;

mod component;
pub use component::{Action, Component, State};

pub mod boxlayout;
pub mod button;
pub mod pages;
pub mod textbox;
pub mod theme;
