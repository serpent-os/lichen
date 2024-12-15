// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

#[allow(dead_code)]
#[allow(unused_imports)]
mod com_serpentos_lichen_disks;
pub mod disks_ipc {
    pub use crate::com_serpentos_lichen_disks::*;
}

pub mod disks;
