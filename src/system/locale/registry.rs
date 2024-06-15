// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Registry of languages and territories.

use std::{collections::HashMap, fs};

use super::{iso_3166, Error};

/// All ISO codes are expected to live in this location
const ISO_CODES_BASE: &str = "/usr/share/iso-codes/json";

/// Manage locales + territories
pub struct Registry {
    places: Vec<Territory>,
    places_lookup: HashMap<String, usize>,
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

impl Registry {
    /// Create a new locale registry from the system iso-code JSON definitions
    pub fn new() -> Result<Self, Error> {
        let places = Self::load_territories()?;
        let mut places_lookup = HashMap::new();
        for (index, item) in places.iter().enumerate() {
            places_lookup.insert(item.code2.to_lowercase(), index);
            places_lookup.insert(item.code.to_lowercase(), index);
        }

        Ok(Self {
            places,
            places_lookup,
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

    /// Retrieve the territory for the given (lower-case) code
    pub fn territory(&self, id: impl AsRef<str>) -> Option<&Territory> {
        if let Some(idx) = self.places_lookup.get(id.as_ref()) {
            self.places.get(*idx)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Registry;

    #[test]
    fn test_new() {
        let r = Registry::new().expect("Failed to initialise registry");
        let ie = r.territory("ie").expect("Cannot find Ireland by ie");
        let irl = r.territory("irl").expect("Cannot find Ireland by irl");
        assert_eq!(ie, irl);
        assert_eq!(irl.display_name, "Ireland");

        let dk = r.territory("dk").expect("Cannot find Denmark by dk");
        assert_eq!(dk.display_name, "Kingdom of Denmark");
    }
}
