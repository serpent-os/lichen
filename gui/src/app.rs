// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Application logic

use cosmic::{
    app::Core,
    executor,
    iced::Length,
    widget::{self, nav_bar},
    Application,
};

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub struct LichenApp {
    core: Core,
    model: nav_bar::Model,
}

static PAGES: [(&str, &str, Page); 2] = [
    ("Welcome", "help-about-symbolic", Page::Welcome),
    (
        "Language",
        "preferences-desktop-locale-symbolic",
        Page::Language,
    ),
];

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Welcome,
    Language,
}

impl Application for LichenApp {
    type Executor = executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.serpentos.lichen";

    fn core(&self) -> &Core {
        &self.core
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.model)
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn view(&self) -> cosmic::prelude::Element<Self::Message> {
        widget::text("Installer")
            .height(Length::Fill)
            .width(Length::Fill)
            .horizontal_alignment(cosmic::iced::alignment::Horizontal::Center)
            .vertical_alignment(cosmic::iced::alignment::Vertical::Center)
            .into()
    }

    fn init(mut core: Core, _flags: Self::Flags) -> (Self, cosmic::app::Command<Self::Message>) {
        core.set_header_title("Installer".into());
        let mut model = nav_bar::Model::builder();
        for (name, icon, data) in PAGES.into_iter() {
            model = model.insert(|p| p.text(name).icon(widget::icon::from_name(icon)).data(data));
        }
        let app = Self {
            model: model.build(),
            core,
        };

        (app, cosmic::app::Command::none())
    }
}
