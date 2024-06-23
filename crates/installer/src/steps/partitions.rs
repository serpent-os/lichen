// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Partititon formatting

use system::disk::Partition;

use super::Context;

/// Format a partition
#[derive(Debug)]
pub struct FormatPartition<'a> {
    /// What partition are we formatting
    pub(crate) partition: &'a Partition,

    /// What filesystem would you like it to have
    pub(crate) filesystem: String,
}

impl<'a> FormatPartition<'a> {
    pub(super) fn execute(&self, _context: &mut Context) {
        let fs = self.filesystem.to_lowercase();
        let command = match fs.as_str() {
            "ext4" => ("mkfs.ext4", [&self.partition.path.display().to_string()]),
            _ => unimplemented!(),
        };
        log::info!("Formatting {} as {}", self.partition.path.display(), self.filesystem);
        log::trace!("Running: {command:?}");
    }

    pub(super) fn title(&self) -> String {
        "Format partition".into()
    }

    pub(super) fn describe(&self) -> String {
        // TODO: More than ext4 xD
        format!("{} as ext4", self.partition.path.display())
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

impl<'a> MountPartition<'a> {
    pub(super) fn execute(&self, _context: &mut Context) {
        log::info!("Mounting {} to {}", self.partition.path.display(), self.mountpoint);
    }

    pub(super) fn title(&self) -> String {
        "Mount filesystem".into()
    }

    pub(super) fn describe(&self) -> String {
        format!("{} as {}", self.partition.path.display(), &self.mountpoint)
    }
}
