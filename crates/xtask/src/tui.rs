use std::ops::ControlFlow;

use futures::{FutureExt as _, StreamExt as _};
use miette::{Context as _, IntoDiagnostic as _};
use ratatui::{
    Frame,
    crossterm::event::{self, Event},
};
use tokio::sync::mpsc;

pub trait TuiApp {
    /// The message type.
    type Message;
    /// The return value of the application.
    type Return;

    /// Handle an event from the terminal.
    async fn handle_event(&mut self, event: Event) -> miette::Result<()>;
    /// Handle a message.
    async fn handle_message(&mut self, message: Self::Message) -> miette::Result<()>;
    /// Update the application state.
    /// Return a control flow to indicate whether the application should continue or break.
    async fn update(&mut self) -> miette::Result<ControlFlow<Self::Return>>;
    /// Render the application state to the terminal.
    fn view(&mut self, frame: &mut Frame) -> miette::Result<()>;
}

pub trait TuiAppExt: TuiApp {
    fn channel(size: usize) -> (mpsc::Sender<Self::Message>, mpsc::Receiver<Self::Message>);

    /// Run the TUI application until completion.
    async fn run(self, receiver: mpsc::Receiver<Self::Message>) -> miette::Result<Self::Return>;
}

async fn run<T: TuiApp, Term: ratatui::backend::Backend>(
    mut app: T,
    mut receiver: mpsc::Receiver<T::Message>,
    mut terminal: ratatui::Terminal<Term>,
) -> miette::Result<T::Return> {
    // Create message/event streams.
    let mut event_stream = event::EventStream::new();
    // Run main loop.
    loop {
        // Handle messages and events.
        {
            // Await either.
            tokio::select! {
                Some(message) = receiver.recv() => {
                    app.handle_message(message).await.wrap_err("handle app message")?;
                }
                Some(Ok(event)) = event_stream.next().fuse() => {
                    app.handle_event(event).await.wrap_err("handle app event")?;
                }
            };
        };
        // Run update.
        let control_flow = app.update().await.wrap_err("app update")?;
        match control_flow {
            ControlFlow::Continue(()) => (),
            ControlFlow::Break(v) => return Ok(v),
        }
        // Run view.
        let mut view_result = None;
        terminal
            .draw(|frame| {
                view_result = Some(app.view(frame).wrap_err("app view"));
            })
            .into_diagnostic()
            .wrap_err("tui render")?;
        // Handle view result.
        let () = view_result
            .expect("draw result should be some")
            .wrap_err("app view")?;
    }
}

impl<T: TuiApp> TuiAppExt for T {
    fn channel(size: usize) -> (mpsc::Sender<Self::Message>, mpsc::Receiver<Self::Message>) {
        mpsc::channel(size)
    }

    async fn run(self, receiver: mpsc::Receiver<Self::Message>) -> miette::Result<Self::Return> {
        // Initialize terminal.
        let terminal = ratatui::init();
        // Run app.
        let return_value = run(self, receiver, terminal).await.wrap_err("run app");
        // Restore terminal.
        ratatui::restore();
        return_value
    }
}
