use std::{mem::ManuallyDrop, time::Instant};

use color_eyre::eyre;
use futures::{future::BoxFuture, stream::BoxStream, Future, FutureExt, Stream, StreamExt};
use tokio::{sync::mpsc, time};

use crate::tui::{event, widget, Element, Event, Screen, Shell};

pub enum Command<Message> {
    Future(BoxFuture<'static, Message>),
    Stream(BoxStream<'static, Message>),
    Quit,
    Focus(Focus),
    Unfocus,
}

pub enum Focus {
    Next,
    Previous,
}

impl<Message> Command<Message> {
    /// Perform an async task
    pub fn perform<T: Send>(
        task: impl Future<Output = T> + Send + 'static,
        f: impl Fn(T) -> Message + Send + 'static,
    ) -> Self {
        Self::Future(task.map(f).boxed())
    }

    /// Run an async stream
    pub fn run<T: Send>(
        stream: impl Stream<Item = T> + Send + 'static,
        f: impl Fn(T) -> Message + Send + 'static,
    ) -> Self {
        Self::Stream(stream.map(f).boxed())
    }

    pub fn focus_next() -> Self {
        Self::Focus(Focus::Next)
    }

    pub fn focus_previous() -> Self {
        Self::Focus(Focus::Previous)
    }

    pub fn unfocus() -> Self {
        Self::Unfocus
    }
}

pub trait Application {
    type Message: Send + 'static;

    /// Handle an event from the Application
    ///
    /// This is called after events are processed by the widget tree. `status`
    /// indicates if any widget captured this event, in which case the application
    /// may want to ignore also processing it
    #[allow(unused_variables)]
    fn handle(&self, event: Event, status: event::Status) -> Option<Self::Message> {
        None
    }
    /// Update the Application with the incoming message
    fn update(&mut self, message: Self::Message) -> Option<Command<Self::Message>>;
    /// Materialize a view with the current state of the Application
    fn view<'a>(&'a self) -> Element<'a, Self::Message>;
}

pub async fn run(mut app: impl Application) -> eyre::Result<()> {
    let mut screen = Screen::new()?;
    screen.run();

    let mut last_screen_size = screen.size()?;

    let mut root = ManuallyDrop::new(app.view());
    let mut layout = root.layout(last_screen_size);

    let (command_sender, mut command_receiver) = mpsc::channel(10);
    let (redraw_sender, mut redraw_receiver) = mpsc::channel(1);

    // Draw initial view
    let _ = redraw_sender.send(()).await;

    let mut focused = None;

    loop {
        let mut shell = Shell::with_focused(focused);
        let mut events = vec![];
        let mut processed_events = vec![];
        let mut redraw = false;

        // Block until an event or command message is received
        tokio::select! {
            event = screen.next_event() => {
                if let Some(event) = event.and_then(Event::from_crossterm) {
                    events.push(event);
                }
            },
            message = command_receiver.recv() => {
                if let Some(message) = message {
                    shell.emit(message);
                }
            }
            _ = redraw_receiver.recv() => {
                events.push(Event::RedrawRequested(Instant::now()));
                redraw = true;
            }
        }

        // Exhaust all immediately available events & command messages
        while let Some(event) = screen.try_next_event().and_then(Event::from_crossterm) {
            events.push(event);
        }
        while let Ok(message) = command_receiver.try_recv() {
            shell.emit(message);
        }

        let current_screen_size = screen.size()?;

        // Ensure layout matches current screen size (resized)
        // before calling update on widget tree
        if last_screen_size != current_screen_size {
            layout = root.layout(current_screen_size);
            shell.request_redraw();
        }

        // Update widget tree w/ events
        for event in events.into_iter() {
            let status = root.update(&layout, event.clone(), &mut shell);
            processed_events.push((event, status));
        }

        // Handle events at application level
        for (event, status) in processed_events {
            if let Some(message) = app.handle(event, status) {
                shell.emit(message);
            }
        }

        // Update app state w/ emitted messages
        if shell.has_messages() {
            let _ = ManuallyDrop::into_inner(root);
            let mut focus = None;

            for message in shell.drain() {
                if let Some(command) = app.update(message) {
                    match command {
                        Command::Future(future) => {
                            let sender = command_sender.clone();
                            tokio::spawn(async move {
                                let _ = sender.send(future.await).await;
                            });
                        }
                        Command::Stream(mut stream) => {
                            let sender = command_sender.clone();
                            tokio::spawn(async move {
                                while let Some(message) = stream.next().await {
                                    let _ = sender.send(message).await;
                                }
                            });
                        }
                        Command::Quit => {
                            screen.stop();
                            return Ok(());
                        }
                        Command::Focus(f) => {
                            focus = Some(f);
                        }
                        Command::Unfocus => {
                            focus.take();
                            shell.unfocus();
                        }
                    }
                }
            }

            // State updated, let's rebuild view, layout and
            // request redraw
            root = ManuallyDrop::new(app.view());
            shell.invalidate_layout();
            shell.request_redraw();

            if let Some(focus) = focus {
                handle_focus(&root, &mut shell, focus);
            }
        }

        if shell.is_layout_invalid() {
            layout = root.layout(current_screen_size);
            shell.request_redraw();
        }

        if let Some(redraw) = shell.requested_redraw() {
            let sender = redraw_sender.clone();
            let duration = redraw.after();
            tokio::spawn(async move {
                time::sleep(duration).await;
                let _ = sender.send(()).await;
            });
        }

        if redraw {
            screen.draw(|f| root.render(f, &layout, shell.focused()))?;
        }

        last_screen_size = current_screen_size;
        focused = shell.focused();
    }
}

fn handle_focus<M>(root: &Element<M>, shell: &mut Shell<M>, focus: Focus) {
    let focusable = root
        .flatten()
        .into_iter()
        .filter_map(|info| {
            if info.attributes.contains(widget::Attributes::FOCUSABLE) && info.id.is_some() {
                info.id
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let focused = focusable.iter().position(|id| Some(*id) == shell.focused());

    let next = match focus {
        Focus::Next => match focused {
            Some(idx) if idx == focusable.len() - 1 => focusable.into_iter().next(),
            None => focusable.into_iter().next(),
            Some(idx) => focusable.into_iter().skip(idx + 1).next(),
        },
        Focus::Previous => match focused {
            None | Some(0) => focusable.into_iter().last(),
            Some(idx) => focusable.into_iter().take(idx).rev().next(),
        },
    };

    if let Some(next) = next {
        shell.focus(next);
    }
}
