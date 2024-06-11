use ratatui::{
    layout::{Constraint, Rect},
    Frame,
};

use crate::tui::{event, Event, Layout, Shell, Widget};

pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message: 'a> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn width(&self) -> Constraint {
        self.widget.width()
    }

    pub fn height(&self) -> Constraint {
        self.widget.height()
    }

    pub fn layout(&self, available: Rect) -> Layout {
        self.widget.layout(available)
    }

    pub fn update(
        &mut self,
        layout: &Layout,
        event: Event,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        self.widget.update(layout, event, shell)
    }

    pub fn render(&self, frame: &mut Frame, layout: &Layout) {
        self.widget.render(frame, layout)
    }

    pub fn map<U: 'a>(self, f: impl Fn(Message) -> U + 'a) -> Element<'a, U> {
        struct Map<'a, T, U> {
            widget: Box<dyn Widget<T> + 'a>,
            mapper: Box<dyn Fn(T) -> U + 'a>,
        }

        impl<'a, T, U> Widget<U> for Map<'a, T, U> {
            fn width(&self) -> Constraint {
                self.widget.width()
            }

            fn height(&self) -> Constraint {
                self.widget.height()
            }

            fn layout(&self, available: Rect) -> Layout {
                self.widget.layout(available)
            }

            fn render(&self, frame: &mut ratatui::prelude::Frame, layout: &Layout) {
                self.widget.render(frame, layout)
            }

            fn update(
                &mut self,
                layout: &Layout,
                event: Event,
                shell: &mut Shell<U>,
            ) -> event::Status {
                let mut local_shell = Shell::default();

                let status = self.widget.update(layout, event, &mut local_shell);

                shell.merge(local_shell.map(&self.mapper));

                status
            }
        }

        Element {
            widget: Box::new(Map {
                widget: self.widget,
                mapper: Box::new(f),
            }),
        }
    }
}
