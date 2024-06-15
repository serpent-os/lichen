// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    IO(#[from] std::io::Error),

    #[error("json parsing error: {0}")]
    Serde(#[from] serde_json::Error),
}

mod iso_3166;
mod iso_639;

mod registry;
