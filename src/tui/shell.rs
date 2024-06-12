use std::time::Duration;

use super::widget;

pub struct Shell<M> {
    relayout: bool,
    redraw: Option<Redraw>,
    messages: Vec<M>,
    focus: Option<Focus>,
}

impl<M> Default for Shell<M> {
    fn default() -> Self {
        Self {
            relayout: false,
            redraw: None,
            messages: vec![],
            focus: None,
        }
    }
}

impl<M> Shell<M> {
    pub fn with_focused(focused: Option<widget::Id>) -> Self {
        Self {
            focus: focused.map(Focus::Set),
            ..Default::default()
        }
    }

    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    pub fn drain(&mut self) -> Vec<M> {
        std::mem::take(&mut self.messages)
    }

    pub fn emit(&mut self, message: M) {
        self.messages.push(message);
    }

    pub fn request_redraw(&mut self) {
        self.redraw = Some(Redraw::Immediately);
    }

    pub fn request_redraw_after(&mut self, duration: Duration) {
        self.redraw = Some(Redraw::After(duration));
    }

    pub fn requested_redraw(&self) -> Option<Redraw> {
        self.redraw
    }

    pub fn invalidate_layout(&mut self) {
        self.relayout = true;
    }

    pub fn is_layout_invalid(&self) -> bool {
        self.relayout
    }

    pub fn map<U>(self, f: impl Fn(M) -> U) -> Shell<U> {
        Shell {
            relayout: self.relayout,
            redraw: self.redraw,
            messages: self.messages.into_iter().map(f).collect(),
            focus: self.focus,
        }
    }

    pub fn merge(&mut self, other: Self) {
        if other.relayout {
            self.relayout = true;
        }
        if let Some(b) = other.redraw {
            if let Some(a) = self.redraw {
                self.redraw = Some(a.merge(b));
            } else {
                self.redraw = Some(b);
            }
        }
        if let Some(focused) = other.focus {
            self.focus = Some(focused);
        }

        self.messages.extend(other.messages);
    }

    pub fn focus(&mut self, widget: widget::Id) {
        self.focus = Some(Focus::Set(widget));
        self.request_redraw();
    }

    pub fn unfocus(&mut self) {
        self.focus = Some(Focus::Unset);
        self.request_redraw();
    }

    pub fn focused(&self) -> Option<widget::Id> {
        match self.focus {
            Some(Focus::Set(id)) => Some(id),
            Some(Focus::Unset) => None,
            None => None,
        }
    }
}

enum Focus {
    Set(widget::Id),
    Unset,
}

#[derive(Debug, Clone, Copy)]
pub enum Redraw {
    Immediately,
    After(Duration),
}

impl Redraw {
    pub fn after(&self) -> Duration {
        if let Self::After(duration) = self {
            *duration
        } else {
            Duration::ZERO
        }
    }

    fn merge(self, other: Redraw) -> Self {
        match (self, other) {
            (Redraw::Immediately, Redraw::Immediately) => self,
            (Redraw::Immediately, Redraw::After(_)) => self,
            (Redraw::After(_), Redraw::Immediately) => other,
            (Redraw::After(a), Redraw::After(b)) => {
                if a < b {
                    self
                } else {
                    other
                }
            }
        }
    }
}
