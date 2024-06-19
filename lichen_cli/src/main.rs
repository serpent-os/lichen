// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Super basic CLI runner for lichen

use std::fmt::Display;

use color_eyre::{eyre::eyre, Section};
use console::{set_colors_enabled, style};
use dialoguer::theme::ColorfulTheme;
use system::{
    disk::{Disk, Partition},
    locale::{self, Locale, Registry},
};
use tokio::process::Command;

/// Installation partitions
#[derive(Debug)]
enum InstallationPartiton {
    /// Boot with optional xbootldr pair
    Boot {
        esp: Partition,
        xbootldr: Option<Partition>,
    },
    /// Standard partition target
    Normal(Partition),
}

impl Display for InstallationPartiton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InstallationPartiton::Boot { esp, xbootldr } => {
                if let Some(xbootldr) = xbootldr.as_ref() {
                    f.write_fmt(format_args!("ESP {} with XBOOTLDR: {}", esp, xbootldr))
                } else {
                    f.write_fmt(format_args!("ESP {}", esp))
                }
            }
            InstallationPartiton::Normal(part) => f.write_fmt(format_args!("Regular partition: {}", part)),
        }
    }
}

/// Craptastic header printing
fn print_header(icon: &str, text: &str) {
    println!("\n\n  {}   {}", style(icon).cyan(), style(text).bright().bold());
    println!("\n\n")
}

/// Crappy print of a summary field
fn print_summary_item(name: &str, item: &impl ToString) {
    let name = console::pad_str(name, 10, console::Alignment::Left, None);
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
fn load_partitions() -> color_eyre::Result<Vec<InstallationPartiton>> {
    let disks = Disk::discover()?;
    let mut partitions = vec![];

    for disk in disks {
        // Ignore errors.
        if let Ok(parts) = disk.partitions() {
            // Find first ESP
            let esp = parts
                .iter()
                .find(|p| matches!(p.kind, system::disk::PartitionKind::ESP));

            // Find first xbootldr
            let xbootldr = parts
                .iter()
                .find(|p| matches!(p.kind, system::disk::PartitionKind::XBOOTLDR));

            if let Some(esp) = esp {
                partitions.push(InstallationPartiton::Boot {
                    esp: esp.clone(),
                    xbootldr: xbootldr.cloned(),
                });
            }

            // Allow "normal partitions" as installation targets
            let regulars = parts
                .iter()
                .filter(|p| matches!(p.kind, system::disk::PartitionKind::Regular));
            partitions.extend(regulars.map(|p| InstallationPartiton::Normal(p.clone())))
        }
    }

    if partitions.is_empty() {
        return Err(eyre!("Cannot find any usable partitions")
            .suggestion("The installer requires that disks are readable and using a GUID Partition Table"));
    }

    // ensure we have an ESP..
    let esp = partitions
        .iter()
        .find(|p| matches!(p, InstallationPartiton::Boot { esp: _, xbootldr: _ }));
    if let Some(InstallationPartiton::Boot { esp, xbootldr }) = esp {
        if xbootldr.is_none() {
            eprintln!("Warning: No XBOOTLDR available for ESP: {}", esp.path.display());
        }
    } else {
        return Err(eyre!("No usable EFI System Partition").suggestion(
            "The installer requires that an ESP is present, and will also use an XBOOTLDR partition if present",
        ));
    }

    Ok(partitions)
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

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    set_colors_enabled(true);

    // Load all the things
    let partitions = load_partitions()?;
    for part in partitions {
        eprintln!("Partition: {}", part);
    }

    // TODO: Make Registry use asynchronous loading
    let registry = locale::Registry::new()?;
    let locales = load_locales(&registry).await?;

    let selected_locale = ask_locale(&locales).await?;
    let timezone = ask_timezone()?;
    let _ = ask_password()?;

    print_header("🕮", "Quickly review your settings");
    print_summary_item("Locale", selected_locale);
    print_summary_item("Timezone", &timezone);

    println!("\n\n");

    let _y = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with installation?")
        .interact()?;

    Ok(())
}
