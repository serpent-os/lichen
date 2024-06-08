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
}
