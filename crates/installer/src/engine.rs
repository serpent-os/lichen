// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Concrete implementation of the isntaller

use system::{
    disk::{self, Disk},
    locale::{self},
};
use thiserror::Error;

use crate::{BootPartition, Model, SystemPartition};

#[derive(Debug, Error)]
pub enum Error {
    #[error("disk: {0}")]
    Disk(#[from] disk::Error),

    #[error("locale: {0}")]
    Locale(#[from] locale::Error),

    #[error("unknown locale code: {0}")]
    UnknownLocale(String),
}

/// The installer does some initial probing and is used with a Model
/// to build an execution routine
pub struct Installer {
    /// Complete locale registry
    locale_registry: locale::Registry,

    /// All known/useful disks
    disks: Vec<Disk>,

    /// Boot partitions
    boot_parts: Vec<BootPartition>,

    /// System partitions
    system_parts: Vec<SystemPartition>,
}

impl Installer {
    /// Return a newly initialised installer
    pub async fn new() -> Result<Self, Error> {
        let locale_registry = locale::Registry::new().await?;
        let disks = Disk::discover().await?;

        let mut boot_parts = vec![];
        let mut system_parts = vec![];
        for disk in disks.iter() {
            if let Ok(parts) = disk.partitions().await {
                if let Some(esp) = parts
                    .iter()
                    .find(|p| matches!(p.kind, disk::PartitionKind::ESP))
                    .cloned()
                {
                    let xbootldr = parts
                        .iter()
                        .find(|p| matches!(p.kind, disk::PartitionKind::XBOOTLDR))
                        .cloned();
                    boot_parts.push(BootPartition { esp, xbootldr })
                }
                let others = parts
                    .iter()
                    .filter(|p| matches!(p.kind, disk::PartitionKind::Regular))
                    .cloned();
                system_parts.extend(others.map(|p| SystemPartition {
                    partition: p,
                    mountpoint: None,
                }));
            }
        }

        Ok(Self {
            locale_registry,
            disks,
            system_parts,
            boot_parts,
        })
    }

    /// Allow access to locale registry (mapping IDs)
    pub fn locales(&self) -> &locale::Registry {
        &self.locale_registry
    }

    /// Return references to the discovered boot partitions
    pub fn boot_partitions(&self) -> &[BootPartition] {
        &self.boot_parts
    }

    /// Return references to the discovered system partitions
    pub fn system_partitions(&self) -> &[SystemPartition] {
        &self.system_parts
    }

    /// build the model into a set of install steps
    pub fn compile_to_steps(&self, _model: &Model) -> Result<(), Error> {
        todo!("dont know how")
    }
}
