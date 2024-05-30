// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Application logic

use cosmic::{app::Core, executor, iced::Length, widget, Application};

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub struct LichenApp {
    core: Core,
}

impl Application for LichenApp {
    type Executor = executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.serpentos.lichen";

    fn core(&self) -> &Core {
        &self.core
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
        let app = Self { core };

        (app, cosmic::app::Command::none())
    }
}
