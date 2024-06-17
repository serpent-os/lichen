// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Partition APIs

use std::path::PathBuf;

use std::fs;

use gpt::partition_types;

/// Partition on a GPT disk
#[derive(Debug)]
pub struct Partition {
    pub path: PathBuf,
    pub kind: Kind,
    pub size: u64,
    pub uuid: String,
}

/// Specialised type of partition
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Kind {
    ESP,
    XBOOTLDR,
    Regular,
}

impl TryFrom<&gpt::partition::Partition> for Partition {
    fn try_from(value: &gpt::partition::Partition) -> Result<Self, Self::Error> {
        let uuid = value.part_guid.hyphenated().to_string();
        let path = fs::canonicalize(format!("/dev/disk/by-partuuid/{}", uuid))?;
        let kind = match value.part_type_guid {
            partition_types::EFI => Kind::ESP,
            partition_types::FREEDESK_BOOT => Kind::XBOOTLDR,
            _ => Kind::Regular,
        };
        let size = value.size()?;
        Ok(Self { path, kind, size, uuid })
    }

    type Error = super::Error;
}
