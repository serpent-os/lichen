// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Package management encapsulation (moss only)
use tokio::process::Command;

use super::Context;

/// Add a repository to the target disk
#[derive(Debug)]
pub struct AddRepo {
    pub(crate) uri: String,
    pub(crate) name: String,
    pub(crate) priority: u64,
}

impl AddRepo {
    /// Basic display title
    pub(super) fn title(&self) -> String {
        format!("Add repo {}", self.name)
    }

    /// Render the action
    pub(super) fn describe(&self) -> String {
        format!("{} (priority {})", self.uri, self.priority)
    }

    /// Run moss against the target, adding a repo
    pub(super) async fn execute(&self, context: &mut Context) -> Result<(), super::Error> {
        let mut cmd = Command::new("moss");
        cmd.arg("-D");
        cmd.arg(&context.root);
        cmd.arg("-y");
        cmd.args(["repo", "add", &self.name, &self.uri, "-p"]);
        cmd.arg(self.priority.to_string());

        // Run,
        let _ = cmd.spawn()?.wait().await?;
        Ok(())
    }
}

/// Install packages to destdir
#[derive(Debug)]
pub struct InstallPackages {
    pub(crate) names: Vec<String>,
}

impl InstallPackages {
    /// Basic display title
    pub(super) fn title(&self) -> String {
        "Install".into()
    }

    /// Render the action
    pub(super) fn describe(&self) -> String {
        "packages to sysroot".into()
    }

    /// Run moss against the target, adding a repo
    pub(super) async fn execute(&self, context: &mut Context) -> Result<(), super::Error> {
        let mut cmd = Command::new("moss");
        cmd.arg("-D");
        cmd.arg(&context.root);
        cmd.arg("install");
        cmd.args(&self.names);

        // Run
        let _ = cmd.spawn()?.wait().await?;
        Ok(())
    }
}
