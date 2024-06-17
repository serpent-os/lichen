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

    #[error("numbers: {0}")]
    Numbers(#[from] ParseIntError),

    #[error("invalid disk")]
    InvalidDisk,
}

mod disk;
pub use disk::Disk;
