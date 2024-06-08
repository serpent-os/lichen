use std::mem::ManuallyDrop;

use color_eyre::eyre;
use futures::{future::BoxFuture, stream::BoxStream, Future, FutureExt, Stream, StreamExt};
use tokio::sync::mpsc;

use crate::{Element, Screen, Shell};

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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    // TODO: Encapsulate Box<Widget> in some type so
    // we don't have to type it
    fn view<'a>(&'a self) -> Element<'a, Self::Message>;
}

pub async fn run(mut app: impl Application) -> eyre::Result<()> {
    let mut screen = Screen::new()?;
    screen.run();

    // Draw initial view
    let mut root = ManuallyDrop::new(app.view());
    screen.draw(|f| root.render(f, f.size()))?;

    let (command_sender, mut command_receiver) = mpsc::channel(10);

    loop {
        let mut shell = Shell::default();

        // Block until an event or command message is received
        tokio::select! {
            event = screen.next_event() => {
                if let Some(event) = event {
                    root.update(event.clone(), &mut shell);
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
            root.update(event.clone(), &mut shell);
        }
        while let Ok(message) = command_receiver.try_recv() {
            shell.emit(message);
        }

        if !shell.messages.is_empty() {
            let _ = ManuallyDrop::into_inner(root);

            for message in shell.messages.drain(..) {
                match app.update(message) {
                    Command::Future(future) => {
                        let sender = command_sender.clone();
                        tokio::spawn(async move {
                            sender.send(future.await).await;
                        });
                    }
                    Command::Stream(mut stream) => {
                        let sender = command_sender.clone();
                        tokio::spawn(async move {
                            while let Some(message) = stream.next().await {
                                sender.send(message).await;
                            }
                        });
                    }
                    Command::Quit => {
                        screen.stop();
                        return Ok(());
                    }
                }
            }

            root = ManuallyDrop::new(app.view());
            shell.request_redraw();
        }

        if shell.redraw {
            screen.draw(|f| root.render(f, f.size()))?;
        }
    }
}
