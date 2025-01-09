// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing for ISO-639 files from iso-codes
//! Essentially, loading of languages

use serde::Deserialize;

use super::Language;

/// JSON document for ISO-639-2
#[derive(Deserialize)]
pub struct Document<'a> {
    #[serde(rename = "639-2", borrow)]
    pub entries: Vec<Entry<'a>>,
}

/// A single entry in the JSON document
#[derive(Deserialize)]
pub struct Entry<'a> {
    #[serde(rename = "alpha_2", borrow)]
    pub code2: Option<&'a str>,

    #[serde(rename = "alpha_3", borrow)]
    pub code3: &'a str,

    /// Official display name
    #[serde(borrow)]
    pub name: &'a str,

    /// Common name (optional)
    #[serde(borrow)]
    pub common_name: Option<&'a str>,

    /// Three letter bibliographic
    pub bibliographic: Option<&'a str>,
}

impl From<&Entry<'_>> for Language {
    /// Convert iso entry into Language
    fn from(value: &Entry<'_>) -> Self {
        Self {
            code: value.code3.into(),
            code2: value.code2.map(|v| v.into()),
            display_name: value.name.into(),
            inverted_name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Document;

    #[test]
    fn load_iso_639_2() {
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

        let loaded = serde_json::from_str::<Document>(TEST_DATA).expect("Failed to decode ISO-639-2 data");
        let ga = loaded
            .entries
            .iter()
            .find(|i| i.code3 == "gle")
            .expect("Failed to find GLE");
        assert_eq!(ga.name, "Irish");
    }
}
