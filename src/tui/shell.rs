use super::widget;

pub struct Shell<M> {
    relayout: bool,
    redraw: bool,
    messages: Vec<M>,
    focus: Option<Focus>,
}

impl<M> Default for Shell<M> {
    fn default() -> Self {
        Self {
            relayout: false,
            redraw: false,
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

    pub fn drain(&mut self) -> impl Iterator<Item = M> + '_ {
        self.messages.drain(..)
    }

    pub fn emit(&mut self, message: M) {
        self.messages.push(message);
    }

    pub fn request_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn is_redraw_requested(&self) -> bool {
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
        if other.redraw {
            self.redraw = true;
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
