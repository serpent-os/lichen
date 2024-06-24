// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installer APIs

mod model;

pub use model::Model;

mod account;
pub use account::Account;

mod engine;
pub use engine::Installer;

mod partitions;
pub use partitions::{BootPartition, SystemPartition};

pub mod systemd;

pub use system::locale::Locale;

pub mod steps;

mod selections;
pub use selections::{Group, Manager};
