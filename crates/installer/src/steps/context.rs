// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installation steps

use std::path::PathBuf;

/// Context for the steps that are executing
pub struct Context {
    root: PathBuf,
}

impl Context {
    /// Create a new context
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}
