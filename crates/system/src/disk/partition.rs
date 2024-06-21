// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Partition APIs

use std::fmt::Display;
use std::path::PathBuf;

use std::fs;

use gpt::disk::LogicalBlockSize;
use gpt::partition_types;

/// Partition on a GPT disk
#[derive(Debug, Clone, Default)]
pub struct Partition {
    pub path: PathBuf,
    pub kind: Kind,
    pub size: u64,
    pub uuid: String,
}

impl Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path.display().to_string())
    }
}
/// Specialised type of partition
#[derive(Debug, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
pub enum Kind {
    ESP,
    XBOOTLDR,
    #[default]
    Regular,
}

impl Partition {
    /// Construct new Partition from the given GPT Partition and block size
    pub fn from(value: &gpt::partition::Partition, block_size: &LogicalBlockSize) -> Result<Self, super::Error> {
        let uuid = value.part_guid.hyphenated().to_string();
        let path = fs::canonicalize(format!("/dev/disk/by-partuuid/{}", uuid))?;
        let kind = match value.part_type_guid {
            partition_types::EFI => Kind::ESP,
            partition_types::FREEDESK_BOOT => Kind::XBOOTLDR,
            _ => Kind::Regular,
        };
        let size = value.bytes_len(*block_size)?;
        Ok(Self { path, kind, size, uuid })
    }
}
