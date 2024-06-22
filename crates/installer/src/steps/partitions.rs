// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Partititon formatting

use system::disk::Partition;

use super::Step;

/// Format a partition
#[derive(Debug)]
pub struct FormatPartition<'a> {
    /// What partition are we formatting
    pub(crate) partition: &'a Partition,

    /// What filesystem would you like it to have
    pub(crate) filesystem: String,
}

impl<'a> Step for FormatPartition<'a> {
    fn name(&self) -> &'static str {
        "format-partition"
    }

    fn execute(&self) {
        todo!()
    }
}

/// Mount a given partition
#[derive(Debug)]
pub struct MountPartition<'a> {
    /// Which partition?
    pub(crate) partition: &'a Partition,

    /// Where are we mounting it?
    pub(crate) mountpoint: String,
}

impl<'a> Step for MountPartition<'a> {
    fn name(&self) -> &'static str {
        "mount-partition"
    }

    fn execute(&self) {
        todo!()
    }
}
