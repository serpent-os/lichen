// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Super basic CLI runner for lichen

use std::{str::FromStr, time::Duration};

use console::{set_colors_enabled, style};
use crossterm::style::Stylize;
use dialoguer::theme::ColorfulTheme;
use indicatif::ProgressStyle;
use installer::{selections, systemd, Account, BootPartition, Installer, Locale, SystemPartition};

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
fn ask_rootfs(parts: &[SystemPartition]) -> color_eyre::Result<&SystemPartition> {
    print_header("", "Please choose a partition to format and install Serpent OS to (/)");
    let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a suitably sized partition")
        .items(parts)
        .default(0)
        .interact()?;
    Ok(&parts[index])
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

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    set_colors_enabled(true);

    // Test selection management, force GNOME
    let selections = selections::Manager::new().with_groups([
        selections::Group::from_str(include_str!("../../selections/base.json"))?,
        selections::Group::from_str(include_str!("../../selections/develop.json"))?,
        selections::Group::from_str(include_str!("../../selections/gnome.json"))?,
        selections::Group::from_str(include_str!("../../selections/kernel-common.json"))?,
        selections::Group::from_str(include_str!("../../selections/kernel-desktop.json"))?,
    ]);

    let load_spinner = indicatif::ProgressBar::new(1)
        .with_message(format!("{}", "Loading".blue()))
        .with_style(
            ProgressStyle::with_template(" {spinner} {wide_msg} ")
                .unwrap()
                .tick_chars("--=≡■≡=--"),
        );
    load_spinner.enable_steady_tick(Duration::from_millis(150));

    // Load all the things
    let inst = Installer::new().await?;
    let boots = inst.boot_partitions();
    let parts = inst.system_partitions();
    let locales = inst.locales_for_ids(systemd::localectl_list_locales().await?).await?;

    load_spinner.finish_and_clear();

    let selected_locale = ask_locale(&locales).await?;
    let timezone = ask_timezone()?;
    let rootpw = ask_password()?;

    let esp = ask_esp(boots)?;

    // Set / partition
    let mut rootfs = ask_rootfs(parts)?.clone();
    rootfs.mountpoint = Some("/".into());

    print_header("🕮", "Quickly review your settings");
    print_summary_item("Locale", selected_locale);
    print_summary_item("Timezone", &timezone);
    print_summary_item("Bootloader", esp);
    print_summary_item("Root (/) filesystem", &rootfs);

    let model = installer::Model {
        accounts: [Account::root().with_password(rootpw)].into(),
        boot_partition: esp.to_owned(),
        partitions: [rootfs.clone()].into(),
        locale: Some(selected_locale),
        timezone: Some(timezone),
    };
    println!("\n\n");

    eprintln!("model: {model:?}");

    let y = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with installation?")
        .interact()?;
    if !y {
        return Ok(());
    }

    // Push some packages into the installer based on selections

    // TODO: Use proper temp directory
    let mut context = installer::steps::Context::new("/tmp/lichen").with_packages(selections.selections_with([
        "develop",
        "gnome",
        "kernel-desktop",
    ])?);

    let (cleanups, steps) = inst.compile_to_steps(&model, &context)?;
    let multi = indicatif::MultiProgress::new();
    let total = indicatif::ProgressBar::new(steps.len() as u64 + cleanups.len() as u64).with_style(
        ProgressStyle::with_template("\n|{bar:20.cyan/blue}| {pos}/{len}")
            .unwrap()
            .progress_chars("■≡=- "),
    );

    let total = multi.add(total);
    for step in steps {
        let progress_bar = multi.insert_before(
            &total,
            indicatif::ProgressBar::new(1)
                .with_message(format!("{} {}", step.title().blue(), step.describe().bold(),))
                .with_style(
                    ProgressStyle::with_template(" {spinner} {wide_msg} ")
                        .unwrap()
                        .tick_chars("--=≡■≡=--"),
                ),
        );
        progress_bar.enable_steady_tick(Duration::from_millis(150));
        total.inc(1);

        // TODO: On a step failure, we tear down context cleanly and dump an error
        step.execute(&mut context).await?;
    }

    // Execute all the cleanups
    for cleanup in cleanups {
        let progress_bar = multi.insert_before(
            &total,
            indicatif::ProgressBar::new(1)
                .with_message(format!("{} {}", cleanup.title().yellow(), cleanup.describe().bold(),))
                .with_style(
                    ProgressStyle::with_template(" {spinner} {wide_msg} ")
                        .unwrap()
                        .tick_chars("--=≡■≡=--"),
                ),
        );
        progress_bar.enable_steady_tick(Duration::from_millis(150));
        total.inc(1);
        cleanup.execute(&mut context).await?;
    }

    Ok(())
}
