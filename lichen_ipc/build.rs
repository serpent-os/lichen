// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

extern crate varlink_generator;

fn main() {
    println!("cargo:rerun-if-changed=src/com.serpentos.lichen.disks.varlink");
    varlink_generator::cargo_build_tosource("src/com.serpentos.lichen.disks.varlink", true);
}
