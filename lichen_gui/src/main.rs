// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use cosmic::{
    app::{Core, Settings},
    cosmic_theme::ThemeBuilder,
    iced::{Length, Size},
    prelude::*,
    widget::{self, icon, nav_bar, JustifyContent},
    Application, Command,
};
use lichen_gui::pages::Page;
use lichen_gui::Message;

pub struct App {
    core: Core,
    model: nav_bar::Model,
    views: Vec<Page>,
    page_num: usize,
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
        };

        let layout: [(&str, &str, Page); 7] = [
            ("Welcome", "go-home-symbolic", Page::welcome()),
            ("Language", "preferences-desktop-locale-symbolic", Page::language()),
            ("Timezone", "preferences-system-time-symbolic", Page::none()),
            ("Disks", "drive-harddisk-symbolic", Page::none()),
            ("Users", "system-users-symbolic", Page::none()),
            ("Selections", "edit-select-all-symbolic", Page::none()),
            ("Review settings", "edit-find-symbolic", Page::none()),
        ];

        for (name, icon_name, page) in layout.into_iter() {
            app.model.insert().text(name).icon(icon::from_name(icon_name));
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
        let back = widget::button::icon(icon::from_name("go-previous-symbolic"))
            .label("Previous")
            .on_press(Message::GoBack);
        let next = widget::button::icon(icon::from_name("go-next-symbolic"))
            .label("Next")
            .on_press(Message::GoForwards);
        let current = self.views.get(self.page_num).unwrap();

        widget::column::with_children(vec![
            current.view(),
            widget::flex_row(vec![back.into(), next.into()])
                .justify_content(JustifyContent::FlexEnd)
                .into(),
        ])
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
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
