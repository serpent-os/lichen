// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Post-installation tasks

use system::locale::Locale;
use tokio::process::Command;

use crate::Account;

use super::{Context, Error};

/// Configure an account on the system
#[derive(Debug)]
pub struct SetPassword<'a> {
    pub(crate) account: &'a Account,
    pub(crate) password: String,
}

impl<'a> SetPassword<'a> {
    pub(super) fn title(&self) -> String {
        "Set account password".to_string()
    }

    pub(super) fn describe(&self) -> String {
        self.account.username.clone()
    }

    /// Execute to configure the account
    pub(super) async fn execute(&self, context: &'a impl Context<'a>) -> Result<(), Error> {
        let mut cmd = Command::new("chroot");
        cmd.arg(context.root().clone());
        cmd.arg("chpasswd");

        let password_text = format!("{}:{}\n", &self.account.username, self.password);
        context.run_command_captured(&mut cmd, Some(&password_text)).await?;

        Ok(())
    }
}

/// Update locale in `locale.conf`
#[derive(Debug)]
pub struct SetLocale<'a> {
    pub(crate) locale: &'a Locale<'a>,
}

impl<'a> SetLocale<'a> {
    pub(super) fn title(&self) -> String {
        "Set system locale".to_string()
    }

    pub(super) fn describe(&self) -> String {
        self.locale.display_name.clone()
    }

    pub(super) async fn execute(&self, context: &'a impl Context<'a>) -> Result<(), Error> {
        let contents = format!("LANG={}\n", self.locale.name);
        let path = context.root().join("etc").join("locale.conf");
        tokio::fs::write(path, &contents).await?;

        Ok(())
    }
}
