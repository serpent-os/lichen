// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Concrete implementation of the isntaller

use system::{
    disk::{self, Disk},
    locale::{self, Locale},
};
use thiserror::Error;

use crate::Model;

#[derive(Debug, Error)]
pub enum Error {
    #[error("disk: {0}")]
    Disk(#[from] disk::Error),

    #[error("locale: {0}")]
    Locale(#[from] locale::Error),
}

/// The installer does some initial probing and is used with a Model
/// to build an execution routine
pub struct Installer<'a> {
    /// Complete locale registry
    locale_registry: locale::Registry,

    /// Available / loaded locales
    locales: Vec<Locale<'a>>,

    /// All known/useful disks
    disks: Vec<Disk>,
}

impl<'a> Installer<'a> {
    /// Return a newly initialised installer
    pub fn new() -> Result<Self, Error> {
        let locale_registry = locale::Registry::new()?;
        let disks = Disk::discover()?;
        Ok(Self {
            locale_registry,
            locales: Vec::new(),
            disks,
        })
    }

    /// build the model into a set of install steps
    pub fn compile_to_steps(&self, _model: &Model) -> Result<(), Error> {
        todo!("dont know how")
    }
}
