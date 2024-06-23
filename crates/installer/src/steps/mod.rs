// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installation steps

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

pub trait Step: Debug {
    /// Unique step name for debugging etc.
    fn name(&self) -> &'static str;

    /// Return presentable, generic title
    fn title(&self) -> String;

    /// Describe the operation more specifically
    fn describe(&self) -> String;

    /// Request execution of the step
    fn execute(&self, context: &mut Context);
}

mod partitions;
use std::{fmt::Debug, path::PathBuf};

pub use partitions::{FormatPartition, MountPartition};
