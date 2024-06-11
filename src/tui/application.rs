use std::mem::ManuallyDrop;

use color_eyre::eyre;
use futures::{future::BoxFuture, stream::BoxStream, Future, FutureExt, Stream, StreamExt};
use tokio::sync::mpsc;

use crate::tui::{event, Element, Event, Screen, Shell};

pub enum Command<Message> {
    Future(BoxFuture<'static, Message>),
    Stream(BoxStream<'static, Message>),
    Quit,
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

    // Draw initial view
    let mut root = ManuallyDrop::new(app.view());
    let mut layout = root.layout(last_screen_size);
    screen.draw(|f| root.render(f, &layout))?;

    let (command_sender, mut command_receiver) = mpsc::channel(10);

    loop {
        let mut shell = Shell::default();
        let mut events = vec![];
        let mut processed_events = vec![];

        // Block until an event or command message is received
        tokio::select! {
            event = screen.next_event() => {
                if let Some(event) = event {
                    events.push(event);
                }
            },
            message = command_receiver.recv() => {
                if let Some(message) = message {
                    shell.emit(message);
                }
            }
        }

        // Exhaust all immediately available events & command messages
        while let Some(event) = screen.try_next_event() {
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
        for event in events.into_iter().filter_map(Event::from_crossterm) {
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
        if !shell.messages.is_empty() {
            let _ = ManuallyDrop::into_inner(root);

            for message in shell.messages.drain(..) {
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
                    }
                }
            }

            // State updated, let's rebuild view, layout and
            // request redraw
            root = ManuallyDrop::new(app.view());
            shell.invalidate_layout();
            shell.request_redraw();
        }

        if shell.relayout {
            layout = root.layout(current_screen_size);
            shell.request_redraw();
        }

        if shell.redraw {
            screen.draw(|f| root.render(f, &layout))?;
        }

        last_screen_size = current_screen_size;
    }
}
