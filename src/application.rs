use std::mem::ManuallyDrop;

use color_eyre::eyre;
use futures::{future::BoxFuture, stream::BoxStream, StreamExt};
use tokio::sync::mpsc;

use crate::{Component, Screen, Shell};

pub enum Command<Message> {
    Future(BoxFuture<'static, Message>),
    Stream(BoxStream<'static, Message>),
}

pub trait Application {
    type Message: Send + 'static;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    // TODO: Encapsulate Box<Component> in some type so
    // we don't have to type it
    fn view<'a>(&'a self) -> Box<dyn Component<Message = Self::Message> + 'a>;
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

        tokio::select! {
            event = screen.next_event() => {
                if let Some(event) = event {
                    root.update(event, &mut shell);
                }
            },
            message = command_receiver.recv() => {
                if let Some(message) = message {
                    shell.emit(message);
                }
            }
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
