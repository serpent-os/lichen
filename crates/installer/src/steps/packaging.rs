// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

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
        format!("{} (priority {}", self.uri, self.name)
    }

    /// Run moss against the target, adding a repo
    pub(super) async fn execute(&self, _context: &mut Context) -> Result<(), super::Error> {
        let mut cmd = Command::new("moss");
        cmd.args(["repo", "add", &self.name, &self.uri, "-p"]);
        cmd.arg(self.priority.to_string());

        // Run, ignore output
        let _ = cmd.output().await?;
        Ok(())
    }
}
