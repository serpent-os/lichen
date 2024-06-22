// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installation steps

pub trait Step: Debug {
    /// Unique step name for debugging etc.
    fn name(&self) -> &'static str;

    /// Request execution of the step
    fn execute(&self);
}

mod partitions;
use std::fmt::Debug;

pub use partitions::{FormatPartition, MountPartition};
