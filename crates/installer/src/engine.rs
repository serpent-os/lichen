// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Concrete implementation of the isntaller

use std::path::Path;

use futures::stream::{self, StreamExt};
use system::{
    disk::{self, Disk},
    locale::{self, Locale},
};
use thiserror::Error;
use tokio::task::JoinError;
use topology::disk::builder::Builder;

use crate::{
    steps::{BindMount, Context, FormatPartition, MountPartition, Step},
    BootPartition, Model, SystemPartition,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("disk: {0}")]
    Disk(#[from] disk::Error),

    #[error("locale: {0}")]
    Locale(#[from] locale::Error),

    #[error("missing mandatory partition: {0}")]
    MissingPartition(&'static str),

    #[error("unknown locale code: {0}")]
    UnknownLocale(String),

    #[error("topology: {0}")]
    Topology(#[from] topology::disk::Error),

    #[error("thread: {0}")]
    Thread(#[from] JoinError),
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

        // Figure out where we live right now and exclude the rootfs
        let probe = Builder::default().build()?;
        let root_nodes = if let Ok(device) = probe.get_rootfs_device("/") {
            let mut nodes = probe.get_device_chain(&device.path).unwrap_or_default();
            nodes.push(device.path.into());
            nodes
        } else {
            vec![]
        };

        // Exclude parent block devices related to `/` partition
        let parents = root_nodes
            .iter()
            .filter_map(|n| probe.get_device_parent(n))
            .collect::<Vec<_>>();

        let mut boot_parts = vec![];
        let mut system_parts = vec![];
        for disk in disks.iter().filter(|d| !parents.iter().any(|r| *r == d.path)) {
            if let Ok(parts) = disk.partitions().await {
                // Exclude partitions related to `/` partition
                let parts = parts
                    .into_iter()
                    .filter(|p| !root_nodes.iter().any(|r| *r == p.path))
                    .collect::<Vec<_>>();
                if let Some(esp) = parts
                    .iter()
                    .find(|p| matches!(p.kind, disk::PartitionKind::ESP))
                    .cloned()
                {
                    let xbootldr = parts
                        .iter()
                        .find(|p| matches!(p.kind, disk::PartitionKind::XBOOTLDR))
                        .cloned();
                    boot_parts.push(BootPartition {
                        esp,
                        xbootldr,
                        parent_desc: disk.to_string(),
                    })
                }
                let others = parts
                    .iter()
                    .filter(|p| matches!(p.kind, disk::PartitionKind::Regular))
                    .cloned();
                system_parts.extend(others.map(|p| SystemPartition {
                    partition: p,
                    mountpoint: None,
                    parent_desc: disk.to_string(),
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

    /// Generate/load the locale map as async stream
    pub async fn locales_for_ids<S: IntoIterator<Item = impl AsRef<str>>>(&self, ids: S) -> Result<Vec<Locale>, Error> {
        let res = stream::iter(ids.into_iter())
            .filter_map(|id| async move { self.locale_registry.locale(id) })
            .collect::<Vec<_>>()
            .await;

        Ok(res)
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
    pub fn compile_to_steps<'a>(&'a self, model: &'a Model, context: &Context) -> Result<Vec<Step<'a>>, Error> {
        let mut s: Vec<Step<'a>> = vec![];
        let boot_part = &model.boot_partition.esp;

        // Mount efi..
        s.push(Step::mount(MountPartition {
            partition: boot_part,
            mountpoint: "/efi".into(),
        }));

        // Mount xbootldr
        if let Some(xbootldr) = model.boot_partition.xbootldr.as_ref() {
            s.push(Step::mount(MountPartition {
                partition: xbootldr,
                mountpoint: "/boot".into(),
            }));
        };

        let root_partition = model
            .partitions
            .iter()
            .find(|p| {
                if let Some(mount) = p.mountpoint.as_ref() {
                    mount == "/"
                } else {
                    false
                }
            })
            .ok_or(Error::MissingPartition("/"))?;

        s.push(Step::format(FormatPartition {
            partition: &root_partition.partition,
            filesystem: "ext4".into(),
        }));
        s.push(Step::mount(MountPartition {
            partition: &root_partition.partition,
            mountpoint: "/".into(),
        }));
        let mounts = self.create_vfs_mounts(&context.root);
        s.extend(mounts);
        Ok(s)
    }

    fn create_vfs_mounts(&self, prefix: &Path) -> Vec<Step> {
        const PARTS: &[(&str, &str); 5] = &[
            ("/dev", "dev"),
            ("/dev/shm", "dev/shm"),
            ("/dev/pts", "dev/pts"),
            ("/proc", "proc"),
            ("/sys", "sys"),
        ];
        PARTS
            .iter()
            .map(|(source, dest)| {
                Step::bind_mount(BindMount {
                    source: source.into(),
                    dest: prefix.join(dest),
                })
            })
            .collect::<Vec<_>>()
    }
}
