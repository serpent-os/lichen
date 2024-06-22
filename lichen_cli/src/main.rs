// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Super basic CLI runner for lichen

use std::fmt::Display;

use color_eyre::{eyre::eyre, Section};
use console::{set_colors_enabled, style};
use dialoguer::theme::ColorfulTheme;
use indicatif::HumanBytes;
use system::{
    disk::{Disk, Partition},
    locale::{self, Locale, Registry},
};
use tokio::process::Command;

/// Special case handling of boot partitions
/// to join them to related XBOOTLDR
#[derive(Debug)]
struct BootPartition {
    esp: Partition,
    xbootldr: Option<Partition>,
}

impl Display for BootPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(xbootldr) = self.xbootldr.as_ref() {
            f.write_fmt(format_args!(
                "ESP on {} ({}) with XBOOTLDR: {} ({})",
                self.esp,
                HumanBytes(self.esp.size),
                xbootldr,
                HumanBytes(xbootldr.size)
            ))
        } else {
            f.write_fmt(format_args!("ESP on {} ({})", self.esp, HumanBytes(self.esp.size)))
        }
    }
}

#[derive(Debug)]
struct TargetPartition(pub Partition);

impl Display for TargetPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Regular partition on {} ({})",
            self.0,
            HumanBytes(self.0.size)
        ))
    }
}

/// Craptastic header printing
fn print_header(icon: &str, text: &str) {
    println!("\n\n  {}   {}", style(icon).cyan(), style(text).bright().bold());
    println!("\n\n")
}

/// Crappy print of a summary field
fn print_summary_item(name: &str, item: &impl ToString) {
    let name = console::pad_str(name, 20, console::Alignment::Left, None);
    println!("      {}   -  {}", style(name).bold(), item.to_string());
}

/// Load all the locales
async fn load_locales(registry: &Registry) -> color_eyre::Result<Vec<Locale>> {
    let output = Command::new("localectl").arg("list-locales").output().await?;
    let text = String::from_utf8(output.stdout)?;

    Ok(text.lines().filter_map(|l| registry.locale(l)).collect::<Vec<_>>())
}

/// Ask the user what locale to use
async fn ask_locale<'a>(locales: &'a [Locale<'a>]) -> color_eyre::Result<&'a Locale> {
    print_header("🌐", "Now, we need to set the default system locale");
    let index = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select a locale")
        .default(0)
        .with_initial_text("english")
        .highlight_matches(true)
        .max_length(20)
        .items(locales)
        .interact()?;
    Ok(&locales[index])
}

// Grab a password for the root account
fn ask_password() -> color_eyre::Result<String> {
    print_header("🔑", "You'll need to set a default root (administrator) password");
    let password = dialoguer::Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Type the password")
        .with_confirmation("Confirm your password", "Those passwords did not match")
        .interact()?;
    Ok(password)
}

/// TODO: Make this actually use more than Disk and be async!
async fn load_partitions() -> color_eyre::Result<(Vec<BootPartition>, Vec<TargetPartition>)> {
    let disks = Disk::discover().await?;
    let mut partitions = vec![];
    let mut boots = vec![];

    for disk in disks {
        // Ignore errors.
        if let Ok(parts) = disk.partitions().await {
            // Find first ESP
            let esp = parts
                .iter()
                .find(|p| matches!(p.kind, system::disk::PartitionKind::ESP));

            // Find first xbootldr
            let xbootldr = parts
                .iter()
                .find(|p| matches!(p.kind, system::disk::PartitionKind::XBOOTLDR));

            if let Some(esp) = esp {
                boots.push(BootPartition {
                    esp: esp.clone(),
                    xbootldr: xbootldr.cloned(),
                })
            }

            // Allow "normal partitions" as installation targets
            let regulars = parts
                .iter()
                .filter(|p| matches!(p.kind, system::disk::PartitionKind::Regular))
                .cloned()
                .map(TargetPartition);
            partitions.extend(regulars);
        }
    }

    if partitions.is_empty() {
        return Err(eyre!("Cannot find any usable partitions")
            .suggestion("The installer requires that disks are readable and using a GUID Partition Table"));
    }

    // ensure we have an ESP..
    if boots.is_empty() {
        Err(eyre!("No usable EFI System Partition").suggestion(
            "The installer requires that an ESP is present, and will also use an XBOOTLDR partition if present",
        ))
    } else {
        Ok((boots, partitions))
    }
}

fn ask_timezone() -> color_eyre::Result<String> {
    print_header("🕒", "Now we need to set the system timezone");

    let index = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Start typing")
        .items(&chrono_tz::TZ_VARIANTS)
        .default(0)
        .highlight_matches(true)
        .max_length(10)
        .interact()?;

    Ok(chrono_tz::TZ_VARIANTS[index].to_string())
}

/// Pick an ESP please...
fn ask_esp(parts: &[BootPartition]) -> color_eyre::Result<&BootPartition> {
    print_header("", "Please choose an EFI System Partition for Serpent OS to use");
    let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a suitably sized ESP")
        .items(parts)
        .default(0)
        .interact()?;
    Ok(&parts[index])
}

/// Where's it going?
fn ask_rootfs(parts: &[TargetPartition]) -> color_eyre::Result<&TargetPartition> {
    print_header("", "Please choose a partition to format and install Serpent OS to (/)");
    let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a suitably sized partition")
        .items(parts)
        .default(0)
        .interact()?;
    Ok(&parts[index])
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    set_colors_enabled(true);

    // Load all the things
    let (boots, parts) = load_partitions().await?;

    // TODO: Make Registry use asynchronous loading
    let registry = locale::Registry::new().await?;
    let locales = load_locales(&registry).await?;

    let selected_locale = ask_locale(&locales).await?;
    let timezone = ask_timezone()?;
    let _ = ask_password()?;

    let esp = ask_esp(&boots)?;
    let rootfs = ask_rootfs(&parts)?;

    print_header("🕮", "Quickly review your settings");
    print_summary_item("Locale", selected_locale);
    print_summary_item("Timezone", &timezone);
    print_summary_item("Bootloader", esp);
    print_summary_item("Root (/) filesystem", rootfs);

    println!("\n\n");

    let _y = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with installation?")
        .interact()?;

    Ok(())
}
