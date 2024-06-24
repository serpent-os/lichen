// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installation steps

use std::{collections::BTreeSet, path::PathBuf};

/// Context for the steps that are executing
pub struct Context {
    pub root: PathBuf,
    mounts: BTreeSet<PathBuf>,
}

impl Context {
    /// Create a new context
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            mounts: BTreeSet::new(),
        }
    }

    /// Push a successful mount into the unmount list
    pub fn push_mount(&mut self, mount: impl Into<PathBuf>) {
        self.mounts.insert(mount.into());
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        for mount in self.mounts.iter().rev() {
            eprintln!("unmount: {mount:?}")
        }
    }
}
