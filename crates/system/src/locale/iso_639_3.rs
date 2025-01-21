// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing for ISO-639-3 JSON files
use serde::Deserialize;

use super::Language;

/// JSON document for 639-3
#[derive(Deserialize)]
pub struct Document<'a> {
    #[serde(rename = "639-3", borrow)]
    pub entries: Vec<Entry<'a>>,
}

/// Language scope
#[derive(Deserialize)]
pub enum Scope {
    #[serde(rename = "I")]
    Individual,

    #[serde(rename = "M")]
    Macrolanguage,

    #[serde(rename = "S")]
    Special,
}

#[derive(Deserialize)]
pub enum Kind {
    #[serde(rename = "A")]
    Ancient,
    #[serde(rename = "C")]
    Constructed,
    #[serde(rename = "E")]
    Extinct,
    #[serde(rename = "H")]
    Historical,
    #[serde(rename = "L")]
    Living,
    #[serde(rename = "S")]
    Special,
}

/// Single entry in the JSON document
#[derive(Deserialize)]
pub struct Entry<'a> {
    /// Three letter code
    #[serde(rename = "alpha_3", borrow)]
    pub code: &'a str,

    /// Sometimes a 2 letter code is present
    #[serde(rename = "alpha_2", borrow)]
    pub code2: Option<&'a str>,

    /// Official name
    #[serde(borrow)]
    pub name: &'a str,

    /// Inverted name
    #[serde(borrow)]
    pub inverted_name: Option<&'a str>,

    /// Scope of the language
    pub scope: Scope,

    /// Type of language
    #[serde(rename = "type")]
    pub kind: Kind,

    /// Three letter bibliographic
    pub bibliographic: Option<&'a str>,

    /// Common name (optional)
    #[serde(borrow)]
    pub common_name: Option<&'a str>,
}

impl From<&Entry<'_>> for Language {
    fn from(value: &Entry<'_>) -> Self {
        let display = if let Some(name) = value.common_name {
            name.into()
        } else {
            value.name.into()
        };
        Self {
            code: value.code.into(),
            code2: value.code2.map(|v| v.into()),
            display_name: display,
            inverted_name: value.inverted_name.map(|v| v.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Document, Kind, Scope};

    #[test]
    fn load_iso_639_3() {
        const TEST_DATA: &str = r#"
        {
          "639-3": [
                {
                "alpha_3": "gld",
                "name": "Nanai",
                "scope": "I",
                "type": "L"
                },
                {
                "alpha_2": "ga",
                "alpha_3": "gle",
                "name": "Irish",
                "scope": "I",
                "type": "L"
                },
                {
                "alpha_2": "gl",
                "alpha_3": "glg",
                "name": "Galician",
                "scope": "I",
                "type": "L"
                }
            ]
        }
        "#;

        let loaded = serde_json::from_str::<Document<'_>>(TEST_DATA).expect("Failed to decode ISO-639-3 data");
        let ga = loaded
            .entries
            .iter()
            .find(|i| i.code == "gle")
            .expect("Failed to find GLE");
        assert!(matches!(ga.scope, Scope::Individual));
        assert!(matches!(ga.kind, Kind::Living));
    }
}
