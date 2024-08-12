// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

mod language;
mod welcome;

use std::fmt::Debug;

use cosmic::{widget::icon, Element};
use installer::Installer;

use crate::Message;

pub enum IconVariant {
    Normal,
    Symbolic,
}

pub struct Plugin {
    // static name for inventory registry
    pub name: &'static str,
    pub page: fn() -> Box<dyn InstallerPage>,
}

pub trait InstallerPage: Send + Debug {
    /// Init using the installer crate
    fn init(&mut self, _installer: &Installer) {}

    /// return the sidebar name
    fn name(&self) -> &str;

    /// return the view
    fn view(&self) -> Element<Message>;

    /// page display title
    fn title(&self) -> &str;

    /// subtitle
    fn subtitle(&self) -> &str;

    /// page icon
    fn icon(&self, variant: IconVariant) -> icon::Named;
}

inventory::collect!(Plugin);
