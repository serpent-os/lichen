// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::collections::BTreeMap;

use cosmic::{
    app::{Core, Settings},
    cosmic_theme::ThemeBuilder,
    iced::{Length, Size},
    prelude::*,
    widget::{self, icon, nav_bar, JustifyContent},
    Application, Command,
};
use lichen_gui::pages::{IconVariant, InstallerPage, Plugin};
use lichen_gui::Message;

pub struct App {
    core: Core,
    model: nav_bar::Model,
    views: Vec<Box<dyn InstallerPage>>,
    page_num: usize,
    state: AppState,
}

enum AppState {
    // running main wizard
    Running,
}

impl App {
    // fancy page header above the page view
    fn page_header<'a>(&'a self, page: &'a dyn InstallerPage) -> Element<Message> {
        widget::column::with_children(vec![
            widget::row::with_children(vec![
                page.icon(IconVariant::Normal).size(96).into(),
                widget::column::with_children(vec![
                    widget::text::title1(page.title().to_owned()).into(),
                    widget::text::title4(page.subtitle().to_owned()).into(),
                ])
                .spacing(8)
                .padding(8)
                .into(),
            ])
            .into(),
            page.view(),
        ])
        .height(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }

    /// Main view for when the application is running
    fn view_running(&self) -> Element<Message> {
        let back = widget::button::icon(icon::from_name("go-previous-symbolic"))
            .label("Previous")
            .on_press(Message::GoBack);
        let next = widget::button::icon(icon::from_name("go-next-symbolic"))
            .label("Next")
            .on_press(Message::GoForwards);
        let current = self.views.get(self.page_num).unwrap();

        widget::column::with_children(vec![
            self.page_header(current.as_ref()),
            widget::flex_row(vec![back.into(), next.into()])
                .justify_content(JustifyContent::FlexEnd)
                .into(),
        ])
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

impl Application for App {
    type Executor = cosmic::executor::multi::Executor;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.serpentos.lichen";

    /// return the core
    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    /// mutable core
    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    /// init the app
    fn init(core: cosmic::app::Core, _flags: Self::Flags) -> (Self, cosmic::app::Command<Self::Message>) {
        // return app with initialised installer
        let mut app = App {
            core,
            model: nav_bar::Model::default(),
            views: vec![],
            page_num: 0,
            state: AppState::Running,
        };

        /*let layout: [(&str, &str, Page); 7] = [
            ("Welcome", "go-home-symbolic", Page::welcome()),
            ("Language", "preferences-desktop-locale-symbolic", Page::language()),
            ("Timezone", "preferences-system-time-symbolic", Page::none()),
            ("Disks", "drive-harddisk-symbolic", Page::none()),
            ("Users", "system-users-symbolic", Page::none()),
            ("Selections", "edit-select-all-symbolic", Page::none()),
            ("Review settings", "edit-find-symbolic", Page::none()),
        ];*/

        let plugins = inventory::iter::<Plugin>
            .into_iter()
            .map(|p| (p.name, p))
            .collect::<BTreeMap<_, _>>();

        static WANTED: [&str; 2] = ["welcome", "language"];

        for item in WANTED {
            let plugin = plugins
                .get(item)
                .unwrap_or_else(|| panic!("failed to locate plugin: {item}"));
            let page = (plugin.page)();
            app.model
                .insert()
                .text(page.name().to_owned())
                .icon(page.icon(IconVariant::Symbolic));
            app.views.push(page);
        }

        app.model.activate_position(0);

        app.set_header_title("Install Serpent OS".into());
        (app, Command::none())
    }

    fn nav_model(&self) -> Option<&cosmic::widget::nav_bar::Model> {
        Some(&self.model)
    }

    fn update(&mut self, message: Self::Message) -> cosmic::app::Command<Self::Message> {
        match message {
            Message::GoForwards => self.page_num += 1,
            Message::GoBack => {
                if self.page_num > 0 {
                    self.page_num -= 1
                }
            }
            Message::LanguagePicked => return Command::none(),
        };

        if self.page_num > self.views.len() - 1 {
            self.page_num = self.views.len() - 1;
        }

        self.model.activate_position(self.page_num as u16);

        eprintln!("page num = {:?}", self.page_num);
        Command::none()
    }

    /// return current view
    fn view(&self) -> Element<Self::Message> {
        match self.state {
            AppState::Running => self.view_running(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ThemeBuilder::light().build();
    let settings = Settings::default()
        .size(Size::new(1024., 768.))
        .antialiasing(true)
        .theme(Theme::custom(theme.into()));
    cosmic::app::run::<App>(settings, ())?;
    Ok(())
}
