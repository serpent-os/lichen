// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

#[allow(
    dead_code,
    elided_lifetimes_in_paths,
    unused_imports,
    unused_qualifications,
    clippy::needless_lifetimes
)]
mod com_serpentos_lichen_disks;
pub mod disks_ipc {
    pub use crate::com_serpentos_lichen_disks::*;
}

pub mod disks;
