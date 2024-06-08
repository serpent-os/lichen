pub struct Shell<M> {
    pub redraw: bool,
    pub messages: Vec<M>,
}

impl<M> Default for Shell<M> {
    fn default() -> Self {
        Self {
            redraw: false,
            messages: vec![],
        }
    }
}

impl<M> Shell<M> {
    pub fn emit(&mut self, message: M) {
        self.messages.push(message);
    }

    pub fn request_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn map<U>(self, f: impl Fn(M) -> U) -> Shell<U> {
        Shell {
            redraw: self.redraw,
            messages: self.messages.into_iter().map(f).collect(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        if other.redraw {
            self.redraw = true;
        }

        self.messages.extend(other.messages);
    }
}
