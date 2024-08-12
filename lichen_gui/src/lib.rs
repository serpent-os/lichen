// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

pub mod pages;

#[derive(Debug, Clone)]
pub enum Message {
    GoBack,
    GoForwards,
    LanguagePicked,
}
