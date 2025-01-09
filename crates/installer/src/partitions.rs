// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! A higher abstraction over partitions for the purposes of
//! installer usage.
//! Quite simply we only care about the difference in a regular
//! partition, and a boot partition.

use std::fmt::Display;

use human_bytes::human_bytes;
use system::disk;

/// A boot partition is an EFI System Partition which may or may
/// not be paired with an `XBOOTLDR` partition, relative to its location
/// on the same GPT disk.
/// This is a requirement per the Boot Loader Specification.
#[derive(Debug, Clone)]
pub struct BootPartition {
    pub(crate) esp: disk::Partition,
    pub(crate) xbootldr: Option<disk::Partition>,
    pub(crate) parent_desc: String,
}

impl Display for BootPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opt_string = if let Some(xbootldr) = self.xbootldr.as_ref() {
            format!(
                "with XBOOTLDR {} ({}) ",
                xbootldr.path.display(),
                human_bytes(xbootldr.size as f64)
            )
        } else {
            "".into()
        };
        f.write_fmt(format_args!(
            "{} ({}) {}[on {}]",
            self.esp.path.display(),
            human_bytes(self.esp.size as f64),
            opt_string,
            self.parent_desc
        ))
    }
}

/// A system partition is simply a regular partition with a specified mountpoint
/// within the root.
#[derive(Debug, Clone)]
pub struct SystemPartition {
    pub(crate) partition: disk::Partition,

    /// Where will it be mounted
    pub mountpoint: Option<String>,

    pub(crate) parent_desc: String,
}

impl Display for SystemPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ({}) [on {}]",
            self.partition.path.display(),
            human_bytes(self.partition.size as f64),
            self.parent_desc
        ))
    }
}

impl AsRef<disk::Partition> for SystemPartition {
    fn as_ref(&self) -> &disk::Partition {
        &self.partition
    }
}
