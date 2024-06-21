// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! A higher abstraction over partitions for the purposes of
//! installer usage.
//! Quite simply we only care about the difference in a regular
//! partition, and a boot partition.

use system::disk;

/// A boot partition is an EFI System Partition which may or may
/// not be paired with an `XBOOTLDR` partition, relative to its location
/// on the same GPT disk.
/// This is a requirement per the Boot Loader Specification.
#[derive(Debug)]
pub struct BootPartition {
    pub(crate) esp: disk::Partition,
    pub(crate) xbootldr: Option<disk::Partition>,
}

/// A system partition is simply a regular partition with a specified mountpoint
/// within the root.
#[derive(Debug)]
pub struct SystemPartition {
    pub(crate) partition: disk::Partition,

    /// Where will it be mounted
    pub(crate) mountpoint: Option<String>,
}

impl AsRef<disk::Partition> for SystemPartition {
    fn as_ref(&self) -> &disk::Partition {
        &self.partition
    }
}
