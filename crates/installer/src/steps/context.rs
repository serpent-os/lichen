// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installation steps

use std::{collections::BTreeSet, path::PathBuf};

/// Context for the steps that are executing
pub struct Context {
    pub root: PathBuf,
    mounts: BTreeSet<PathBuf>,
    pub(crate) packages: BTreeSet<String>,
}

impl Context {
    /// Create a new context
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            mounts: BTreeSet::new(),
            packages: BTreeSet::new(),
        }
    }

    /// Push a successful mount into the unmount list
    pub fn push_mount(&mut self, mount: impl Into<PathBuf>) {
        self.mounts.insert(mount.into());
    }

    pub fn with_packages<I: IntoIterator<Item = impl AsRef<str>>>(self, pkgs: I) -> Self {
        Self {
            packages: pkgs
                .into_iter()
                .map(|p| p.as_ref().to_string())
                .collect::<BTreeSet<_>>(),
            ..self
        }
    }
}
