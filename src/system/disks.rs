// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Disk management

use std::{fs, io, num::ParseIntError, path::PathBuf};

use thiserror::Error;

/// Indicates type of disk device
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Kind {
    /// Hard disk drive
    HDD,

    /// Solid State device
    SSD,
}

// Basic physical device mapping
#[derive(Debug)]
pub struct Disk {
    pub path: PathBuf,
    pub kind: Kind,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub block_size: u64,
    pub size: u64,
}

/// Error reporting for disks
#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    IO(#[from] io::Error),

    #[error("numbers: {0}")]
    Numbers(#[from] ParseIntError),
}

/// Discover usable root-level disks
fn discover_disks() -> Result<Vec<Disk>, Error> {
    let mut disks = vec![];
    for i in fs::read_dir("/sys/class/block")? {
        let entry = i?;
        let device_link = entry.path().join("device");
        let slavedir = entry.path().join("slaves");

        // Ensure the device link is present (no virtual ram0 device, etc)
        if !device_link.exists() {
            continue;
        }

        // Root level devices, not interested in child partitions as yet.
        let ancestors = fs::read_dir(slavedir)?.filter_map(|m| m.ok()).collect::<Vec<_>>();
        if !ancestors.is_empty() {
            continue;
        }

        // SSD or HDD?
        let rotational = entry.path().join("queue").join("rotational");
        let kind = if rotational.exists() {
            match str::parse::<i32>(fs::read_to_string(rotational)?.trim())? {
                0 => Kind::SSD,
                _ => Kind::HDD,
            }
        } else {
            Kind::HDD
        };

        // additioal metadata.

        let vendor = fs::read_to_string(device_link.join("vendor"))
            .ok()
            .map(|f| f.trim().to_string());
        let model = fs::read_to_string(device_link.join("model"))
            .ok()
            .map(|f| f.trim().to_string());
        let block_size =
            str::parse::<u64>(fs::read_to_string(entry.path().join("queue").join("physical_block_size"))?.trim())?;
        let size = str::parse::<u64>(fs::read_to_string(entry.path().join("size"))?.trim())?;

        let path = PathBuf::from("/dev").join(entry.file_name());

        let disk = Disk {
            path,
            kind,
            vendor,
            model,
            block_size,
            size,
        };
        disks.push(disk);
    }

    Ok(disks)
}
