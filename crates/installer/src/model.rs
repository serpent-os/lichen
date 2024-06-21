// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::collections::BTreeSet;

use crate::{Account, BootPartition, SystemPartition};

/// Core model for the installation target
#[derive(Debug)]
pub struct Model {
    /// All accounts in the system.
    pub accounts: BTreeSet<Account>,

    /// The boot partition to use
    pub boot_partition: BootPartition,

    /// The system partitions to use/mount
    pub partitions: Vec<SystemPartition>,
}
