// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::collections::BTreeSet;

use crate::Account;

/// Core model for the installation target
#[derive(Debug, Default)]
pub struct Model {
    /// All accounts in the system.
    pub accounts: BTreeSet<Account>,
}
