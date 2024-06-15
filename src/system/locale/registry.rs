// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Registry of languages and territories.

use std::{collections::HashMap, fs};

use super::{iso_3166, iso_639, Error};

/// All ISO codes are expected to live in this location
const ISO_CODES_BASE: &str = "/usr/share/iso-codes/json";

/// Manage locales + territories
pub struct Registry {
    places: Vec<Territory>,
    places_lookup: HashMap<String, usize>,
    languages: Vec<Language>,
    languages_lookup: HashMap<String, usize>,
}

/// Sane representation for UI purposes
#[derive(PartialEq, Eq, Debug)]
pub struct Territory {
    pub code: String,
    pub code2: String,
    pub display_name: String,
    pub flag: String,
}

impl From<&iso_3166::Entry<'_>> for Territory {
    fn from(value: &iso_3166::Entry) -> Self {
        if let Some(display) = value.official_name {
            Self {
                code: value.code3.into(),
                code2: value.code2.into(),
                display_name: display.into(),
                flag: value.flag.into(),
            }
        } else {
            Self {
                code: value.code3.into(),
                code2: value.code2.into(),
                display_name: value.name.into(),
                flag: value.flag.into(),
            }
        }
    }
}

/// Simplistic language representation
#[derive(PartialEq, Eq, Debug)]
pub struct Language {
    pub code: String,
    pub code2: Option<String>,
    pub display_name: String,
    pub inverted_name: Option<String>,
}

impl From<&iso_639::EntryTwoCode<'_>> for Language {
    /// Convert iso entry into Language
    fn from(value: &iso_639::EntryTwoCode<'_>) -> Self {
        Self {
            code: value.code3.into(),
            code2: value.code2.map(|v| v.into()),
            display_name: value.name.into(),
            inverted_name: None,
        }
    }
}

impl From<&iso_639::EntryThreeCode<'_>> for Language {
    fn from(value: &iso_639::EntryThreeCode<'_>) -> Self {
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

impl Registry {
    /// Create a new locale registry from the system iso-code JSON definitions
    pub fn new() -> Result<Self, Error> {
        let places = Self::load_territories()?;
        let mut places_lookup = HashMap::new();
        for (index, item) in places.iter().enumerate() {
            places_lookup.insert(item.code2.to_lowercase(), index);
            places_lookup.insert(item.code.to_lowercase(), index);
        }

        //  Convert all languages into usable ones with mapping
        let mut languages = Self::load_languages_2()?;
        languages.extend(Self::load_languages_3()?);
        let mut languages_lookup = HashMap::new();
        for (index, language) in languages.iter().enumerate() {
            if let Some(code2) = language.code2.as_ref() {
                languages_lookup.insert(code2.to_lowercase(), index);
            }
            languages_lookup.insert(language.code.to_lowercase(), index);
        }

        Ok(Self {
            places,
            places_lookup,
            languages,
            languages_lookup,
        })
    }

    /// Load all the territories
    fn load_territories() -> Result<Vec<Territory>, Error> {
        // Load the territories in
        let territories = format!("{}/iso_3166-1.json", ISO_CODES_BASE);
        let contents = fs::read_to_string(territories)?;
        let parser = serde_json::from_str::<iso_3166::Document>(&contents)?;

        Ok(parser.entries.iter().map(|e| e.into()).collect::<Vec<_>>())
    }

    /// Load the 2 DB
    fn load_languages_2() -> Result<Vec<Language>, Error> {
        let languages = format!("{}/iso_639-2.json", ISO_CODES_BASE);
        let contents = fs::read_to_string(languages)?;
        let parser = serde_json::from_str::<iso_639::DocumentTwoCode>(&contents)?;

        Ok(parser.entries.iter().map(|e| e.into()).collect::<Vec<_>>())
    }

    /// Load the 3 DB
    fn load_languages_3() -> Result<Vec<Language>, Error> {
        let languages = format!("{}/iso_639-3.json", ISO_CODES_BASE);
        let contents = fs::read_to_string(languages)?;
        let parser = serde_json::from_str::<iso_639::DocumentThreeCode>(&contents)?;

        Ok(parser.entries.iter().map(|e| e.into()).collect::<Vec<_>>())
    }

    /// Retrieve the territory for the given (lower-case) code
    pub fn territory(&self, id: impl AsRef<str>) -> Option<&Territory> {
        if let Some(idx) = self.places_lookup.get(id.as_ref()) {
            self.places.get(*idx)
        } else {
            None
        }
    }

    /// Retrieve the language for the given (lower-case) code
    pub fn language(&self, id: impl AsRef<str>) -> Option<&Language> {
        if let Some(idx) = self.languages_lookup.get(id.as_ref()) {
            self.languages.get(*idx)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Registry;

    #[test]
    fn test_territory() {
        let r = Registry::new().expect("Failed to initialise registry");
        let ie = r.territory("ie").expect("Cannot find Ireland by ie");
        let irl = r.territory("irl").expect("Cannot find Ireland by irl");
        assert_eq!(ie, irl);
        assert_eq!(irl.display_name, "Ireland");

        let dk = r.territory("dk").expect("Cannot find Denmark by dk");
        assert_eq!(dk.display_name, "Kingdom of Denmark");
    }

    #[test]
    fn test_language() {
        let r = Registry::new().expect("Failed to initialise registry");
        let en = r.language("en").expect("Cannot find English by en");
        assert_eq!(en.display_name, "English");

        let dan = r.language("dan").expect("Cannot find Danish by dan");
        let dn = r.language("da").expect("Cannot find Danish by dn");
        assert_eq!(dan, dn);
    }
}
