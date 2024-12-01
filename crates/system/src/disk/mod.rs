// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Disk management

use std::{io, num::ParseIntError};

use thiserror::Error;

/// Error reporting for disks
#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    IO(#[from] io::Error),

    #[error("gpt: {0}")]
    Gpt(#[from] gpt::GptError),

    #[error("numbers: {0}")]
    Numbers(#[from] ParseIntError),

    #[error("invalid disk")]
    InvalidDisk,

    #[error("superblock: {0}")]
    Superblock(#[from] superblock::Error),
}

mod disks;
pub use disks::Disk;
mod partition;
pub use partition::Kind as PartitionKind;
pub use partition::Partition;

pub use superblock::Kind as SuperblockKind;
