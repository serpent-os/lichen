// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::env;

use clap::Parser;
use color_eyre::eyre::bail;
use lichen_ipc::{disks, disks_ipc};
use pretty_env_logger::formatted_builder;
use varlink::VarlinkService;

/// Command line arguments parser
#[derive(Parser)]
struct Cli {
    /// Varlink socket address
    #[clap(long)]
    varlink: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    color_eyre::install().unwrap();

    let args = Cli::parse();
    let socket = if let Some(varlink) = args.varlink {
        varlink
    } else if let Ok(varlink_address) = env::var("VARLINK_ADDRESS") {
        varlink_address
    } else {
        bail!("Usage: lichen-ipc --varlink <socket>");
    };

    // bind our interfaces to the varlink service
    let interface = disks_ipc::new(Box::new(disks::Service::new()));
    let service = VarlinkService::new(
        "Serpent OS",
        "Lichen Installer",
        "0.1",
        "https://serpentos.com/",
        vec![Box::new(interface)],
    );

    log::info!("lichen-ipc now listening on {socket}");

    varlink::listen(
        service,
        &socket,
        &varlink::ListenConfig {
            idle_timeout: 0,
            ..Default::default()
        },
    )?;

    Ok(())
}
