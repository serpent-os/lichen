// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

fn main() {
    println!("cargo:rerun-if-changed=src/com.serpentos.lichen.disks.varlink");
    varlink_generator::cargo_build_tosource("src/com.serpentos.lichen.disks.varlink", true);
}
