// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen installation steps

mod context;
pub use context::Context;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug)]
pub enum Step<'a> {
    AddRepo(Box<packaging::AddRepo>),
    Bind(Box<partitions::BindMount>),
    Format(Box<partitions::FormatPartition<'a>>),
    Install(Box<packaging::InstallPackages>),
    Mount(Box<partitions::MountPartition<'a>>),
}

impl<'a> Step<'a> {
    /// Create new repo step
    pub fn add_repo(r: packaging::AddRepo) -> Self {
        Self::AddRepo(Box::new(r))
    }

    pub fn install_packages(p: packaging::InstallPackages) -> Self {
        Self::Install(Box::new(p))
    }

    /// Create new FormatPartition step
    pub fn format(f: partitions::FormatPartition<'a>) -> Self {
        Self::Format(Box::new(f))
    }

    /// Create new MountPartition step
    pub fn mount(m: partitions::MountPartition<'a>) -> Self {
        Self::Mount(Box::new(m))
    }

    /// Create new bind mount
    pub fn bind_mount(b: partitions::BindMount) -> Self {
        Self::Bind(Box::new(b))
    }

    /// Return a unique short ID name for the steps
    pub fn name(&self) -> &'static str {
        match &self {
            Step::AddRepo(_) => "add-repo",
            Step::Bind(_) => "bind-mount",
            Step::Format(_) => "format-partition",
            Step::Install(_) => "install-packages",
            Step::Mount(_) => "mount-partition",
        }
    }

    /// Return the display title for a step
    pub fn title(&self) -> String {
        match &self {
            Step::AddRepo(s) => s.title(),
            Step::Bind(s) => s.title(),
            Step::Format(s) => s.title(),
            Step::Install(s) => s.title(),
            Step::Mount(s) => s.title(),
        }
    }

    /// Describe the action/context for the step
    pub fn describe(&self) -> String {
        match &self {
            Step::AddRepo(s) => s.describe(),
            Step::Bind(s) => s.describe(),
            Step::Format(s) => s.describe(),
            Step::Install(s) => s.describe(),
            Step::Mount(s) => s.describe(),
        }
    }

    /// Execute a step asynchronously. Implementations can opt-in to async.
    pub async fn execute(&self, context: &'a impl Context<'a>) -> Result<(), Error> {
        match &self {
            Step::AddRepo(s) => Ok(s.execute(context).await?),
            Step::Bind(s) => Ok(s.execute(context).await?),
            Step::Format(s) => Ok(s.execute(context).await?),
            Step::Install(s) => Ok(s.execute(context).await?),
            Step::Mount(s) => Ok(s.execute(context).await?),
        }
    }
}

mod partitions;
use std::fmt::Debug;

pub use partitions::{BindMount, FormatPartition, MountPartition, Unmount};

mod packaging;
pub use packaging::{AddRepo, InstallPackages};

mod cleanup;
pub use cleanup::Cleanup;
