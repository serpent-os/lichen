// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Super basic CLI runner for lichen

use console::{set_colors_enabled, style};
use dialoguer::theme::ColorfulTheme;
use system::{disk::Disk, locale};
use tokio::process::Command;

fn print_header(icon: &str, text: &str) {
    println!("\n\n  {}   {}", style(icon).cyan(), style(text).bright().bold());
    println!("\n\n")
}

fn print_summary_item(name: &str, item: &impl ToString) {
    let name = console::pad_str(name, 10, console::Alignment::Left, None);
    println!("      {}   -  {}", style(name).bold(), item.to_string());
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    set_colors_enabled(true);

    print_header("🖴", "We need to pick a location to install Serpent OS");
    let disks = Disk::discover()?;
    let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a disk from the following list")
        .items(&disks)
        .default(0)
        .interact()?;

    let selected = &disks[index];

    let reg = locale::Registry::new()?;
    let output = Command::new("localectl").arg("list-locales").output().await?;
    let locales = String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|l| reg.locale(l))
        .collect::<Vec<_>>();

    print_header("🌐", "Now, we need to set the default system locale");
    let l_index = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select a locale")
        .default(0)
        .with_initial_text("english")
        .highlight_matches(true)
        .max_length(20)
        .items(&locales)
        .interact()?;

    let selected_locale = &locales[l_index];

    print_header("🕮", "Quickly review your settings");
    print_summary_item("Disk", selected);
    print_summary_item("Locale", selected_locale);

    println!("\n\n");

    let _y = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with installation?")
        .interact()?;

    Ok(())
}
