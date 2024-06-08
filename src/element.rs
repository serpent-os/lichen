use ratatui::{layout::Rect, Frame};

use crate::{event, Event, Shell, State, Widget};

pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message: 'a> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn update(&mut self, event: Event, shell: &mut Shell<Message>) -> event::Status {
        self.widget.update(event, shell)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        self.widget.render(frame, area)
    }

    pub fn map<U: 'a>(self, f: impl Fn(Message) -> U + 'a) -> Element<'a, U> {
        struct Map<'a, T, U> {
            widget: Box<dyn Widget<T> + 'a>,
            mapper: Box<dyn Fn(T) -> U + 'a>,
        }

        impl<'a, T, U> Widget<U> for Map<'a, T, U> {
            fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
                self.widget.render(frame, area)
            }

            fn update(&mut self, event: Event, shell: &mut Shell<U>) -> event::Status {
                let mut local_shell = Shell::default();

                let status = self.widget.update(event, &mut local_shell);

                shell.merge(local_shell.map(&self.mapper));

                status
            }

            fn state(&self) -> State {
                self.widget.state()
            }

            fn push_state(&self, st: State) {
                self.widget.push_state(st)
            }

            fn pop_state(&self, st: State) {
                self.widget.pop_state(st)
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
