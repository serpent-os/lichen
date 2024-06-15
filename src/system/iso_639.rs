// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing for ISO-639 files from iso-codes
//! Essentially, loading of languages

use serde::Deserialize;

/// Wrap the document stream from JSON into referenced
/// entries in the input text
#[derive(Deserialize)]
pub struct DocumentTwoCode<'a> {
    #[serde(rename = "639-2", borrow)]
    entries: Vec<EntryTwoCode<'a>>,
}

/// A two-letter code entry
#[derive(Deserialize)]
pub struct EntryTwoCode<'a> {
    #[serde(rename = "alpha_2", borrow)]
    code2: &'a str,

    #[serde(rename = "alpha_3", borrow)]
    code3: Option<&'a str>,

    /// Official display name
    #[serde(borrow)]
    name: &'a str,
}

#[derive(Deserialize)]
pub struct DocumentThreeCode<'a> {
    #[serde(rename = "639-3", borrow)]
    entries: Vec<EntryThreeCode<'a>>,
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

#[derive(Deserialize)]
pub struct EntryThreeCode<'a> {
    /// Three letter code
    #[serde(rename = "alpha_3", borrow)]
    code: &'a str,

    /// Sometimes a 2 letter code is present
    #[serde(rename = "alpha_2", borrow)]
    code2: Option<&'a str>,

    /// Official name
    #[serde(borrow)]
    name: &'a str,

    /// Inverted name
    #[serde(borrow)]
    inverted_name: Option<&'a str>,

    /// Scope of the language
    scope: Scope,

    /// Type of language
    #[serde(rename = "type")]
    kind: Kind,

    /// Three letter bibliographic
    bibliographic: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use crate::system::iso_639::{DocumentThreeCode, DocumentTwoCode, Kind, Scope};

    #[test]
    fn load_2() {
        const TEST_DATA: &str = r#"
        {
          "639-2": [
                {
                "alpha_2": "gd",
                "alpha_3": "gla",
                "name": "Gaelic; Scottish Gaelic"
                },
                {
                "alpha_2": "ga",
                "alpha_3": "gle",
                "name": "Irish"
                },
                {
                "alpha_2": "gl",
                "alpha_3": "glg",
                "name": "Galician"
                }
            ]
        }
        "#;

        let loaded = serde_json::from_str::<DocumentTwoCode>(TEST_DATA)
            .expect("Failed to decode ISO-639 2-code data");
        let ga = loaded
            .entries
            .iter()
            .find(|i| i.code2 == "ga")
            .expect("Failed to find GLE");
        assert_eq!(ga.name, "Irish");
    }

    #[test]
    fn load_3() {
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

        let loaded = serde_json::from_str::<DocumentThreeCode>(TEST_DATA)
            .expect("Failed to decode ISO-639 3-code data");
        let ga = loaded
            .entries
            .iter()
            .find(|i| i.code == "gle")
            .expect("Failed to find GLE");
        assert!(matches!(ga.scope, Scope::Individual));
        assert!(matches!(ga.kind, Kind::Living));
    }
}
