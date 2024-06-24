// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Partititon formatting

use std::path::PathBuf;

use system::disk::Partition;
use tokio::process::Command;

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
    pub(super) async fn execute(&self, _context: &mut Context) -> Result<(), super::Error> {
        let fs = self.filesystem.to_lowercase();
        let (exec, args) = match fs.as_str() {
            "ext4" => ("mkfs.ext4", [&self.partition.path.display().to_string()]),
            _ => unimplemented!(),
        };
        log::info!("Formatting {} as {}", self.partition.path.display(), self.filesystem);
        log::trace!("Running: {exec:?} w/ {args:?}");

        // For now we drop output, but we'll wire up stdout/stderr in context
        let _ = Command::new(exec).args(args).output().await?;
        Ok(())
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
    pub(super) async fn execute(&self, _context: &mut Context) -> Result<(), super::Error> {
        log::info!("Mounting {} to {}", self.partition.path.display(), self.mountpoint);
        Ok(())
    }

    pub(super) fn title(&self) -> String {
        "Mount filesystem".into()
    }

    pub(super) fn describe(&self) -> String {
        format!("{} as {}", self.partition.path.display(), &self.mountpoint)
    }
}

/// Bind mount a source dir into a target dir
#[derive(Debug)]
pub struct BindMount {
    /// The source directory
    pub(crate) source: PathBuf,

    /// Destination directory
    pub(crate) dest: PathBuf,
}

impl BindMount {
    pub(super) async fn execute(&self, _context: &mut Context) -> Result<(), super::Error> {
        log::info!("Bind mounting {} to {}", self.source.display(), self.dest.display());
        Ok(())
    }

    pub(super) fn title(&self) -> String {
        "Bind mount filesystem".into()
    }

    pub(super) fn describe(&self) -> String {
        format!("{} on {}", self.source.display(), self.dest.display())
    }
}
