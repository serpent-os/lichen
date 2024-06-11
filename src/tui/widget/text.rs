use std::borrow::Cow;

use ratatui::layout::{Constraint, Rect};

use crate::tui::{event, Element, Event, Layout, Shell, Widget};

pub fn text<'a>(content: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text::new(content)
}

pub struct Text<'a> {
    content: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl<'a, Message: 'a> Widget<Message> for Text<'a> {
    fn width(&self, _height: u16) -> Constraint {
        Constraint::Length(self.content.as_ref().chars().count() as u16)
    }

    fn height(&self, _width: u16) -> Constraint {
        Constraint::Length(1)
    }

    fn layout(&self, available: Rect) -> Layout {
        Layout {
            area: Rect {
                width: (self.content.as_ref().chars().count() as u16).min(available.width),
                height: 1.min(available.height),
                ..available
            },
            children: vec![],
        }
    }

    fn update(
        &mut self,
        _layout: &Layout,
        _event: Event,
        _shell: &mut Shell<Message>,
    ) -> event::Status {
        event::Status::Ignored
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout) {
        frame.render_widget(ratatui::text::Text::raw(self.content.as_ref()), layout.area)
    }
}

impl<'a, Message> From<Text<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: Text<'a>) -> Self {
        Element::new(value)
    }
}

impl<'a, Message> From<&'a str> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: &'a str) -> Self {
        text(value).into()
    }
}
