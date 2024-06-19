// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Super basic CLI runner for lichen

use console::{set_colors_enabled, style};
use dialoguer::theme::ColorfulTheme;
use system::{
    disk::Disk,
    locale::{self, Locale, Registry},
};
use tokio::process::Command;

fn print_header(icon: &str, text: &str) {
    println!("\n\n  {}   {}", style(icon).cyan(), style(text).bright().bold());
    println!("\n\n")
}

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
fn load_disks() -> color_eyre::Result<Vec<Disk>> {
    Ok(Disk::discover()?)
}

/// Ask the user where to install
fn ask_disk(disks: &[Disk]) -> color_eyre::Result<&Disk> {
    print_header("🖴", "We need to pick a location to install Serpent OS");

    let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a disk from the following list")
        .items(disks)
        .default(0)
        .interact()?;

    Ok(&disks[index])
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    set_colors_enabled(true);

    // Load all the things
    let disks = load_disks()?;
    // TODO: Make Registry use asynchronous loading
    let registry = locale::Registry::new()?;
    let locales = load_locales(&registry).await?;

    let selected_disk = ask_disk(&disks)?;
    let selected_locale = ask_locale(&locales).await?;
    let _ = ask_password()?;

    print_header("🕮", "Quickly review your settings");
    print_summary_item("Disk", selected_disk);
    print_summary_item("Locale", selected_locale);

    println!("\n\n");

    let _y = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with installation?")
        .interact()?;

    Ok(())
}
