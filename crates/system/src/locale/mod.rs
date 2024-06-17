// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::fmt;

use thiserror::Error;

/// Locale joins Territory + Language
#[derive(Debug)]
pub struct Locale<'a> {
    pub name: String,
    pub display_name: String,
    pub language: &'a Language,
    pub territory: &'a Territory,
    pub modifier: Option<String>,
    pub codeset: Option<String>,
}

impl fmt::Display for Locale<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display_name)
    }
}

/// Sane representation for UI purposes
#[derive(PartialEq, Eq, Debug)]
pub struct Territory {
    pub code: String,
    pub code2: String,
    pub display_name: String,
    pub flag: String,
}

/// Simplistic language representation
#[derive(PartialEq, Eq, Debug)]
pub struct Language {
    pub code: String,
    pub code2: Option<String>,
    pub display_name: String,
    pub inverted_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    IO(#[from] std::io::Error),

    #[error("json parsing error: {0}")]
    Serde(#[from] serde_json::Error),
}

mod iso_3166;
mod iso_639_2;
mod iso_639_3;

mod registry;
pub use registry::Registry;
