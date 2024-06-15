// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing for ISO-3166-1 files from iso-codes
//! Essentially, loading of territories.

use serde::Deserialize;

/// Wrap the document stream from JSON into referenced
/// entries in the input text
#[derive(Deserialize)]
pub struct Document<'a> {
    #[serde(rename = "3166-1", borrow)]
    entries: Vec<Entry<'a>>,
}

/// Maps an entry from iso-codes to a Rusty struct.
#[derive(Deserialize)]
pub struct Entry<'a> {
    /// Two-element code identifying the entry
    #[serde(rename = "alpha_2", borrow)]
    code2: &'a str,

    /// Three-element code identifying the entry
    #[serde(rename = "alpha_3", borrow)]
    code3: &'a str,

    /// Unicode flag representation
    #[serde(borrow)]
    flag: &'a str,

    /// Normalised name
    #[serde(borrow)]
    name: &'a str,

    /// Unique territory
    #[serde(borrow)]
    numeric: &'a str,

    /// Formal name if present
    #[serde(borrow)]
    official_name: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::Document;

    #[test]
    fn basic_load() {
        const TEST_DATA: &str = r#"
        {
            "3166-1": [

                {
                "alpha_2": "IN",
                "alpha_3": "IND",
                "flag": "🇮🇳",
                "name": "India",
                "numeric": "356",
                "official_name": "Republic of India"
                },
                {
                "alpha_2": "IO",
                "alpha_3": "IOT",
                "flag": "🇮🇴",
                "name": "British Indian Ocean Territory",
                "numeric": "086"
                },
                {
                "alpha_2": "IE",
                "alpha_3": "IRL",
                "flag": "🇮🇪",
                "name": "Ireland",
                "numeric": "372"
                }
            ]
        }
          "#;
        let loaded =
            serde_json::from_str::<Document>(TEST_DATA).expect("Failed to decode ISO-3166 JSON");

        let ie = loaded
            .entries
            .iter()
            .find(|e| e.code2 == "IE")
            .expect("Failed to find locale");
        assert_eq!(ie.name, "Ireland");
        eprintln!("Ireland: {}", ie.flag);
    }
}
