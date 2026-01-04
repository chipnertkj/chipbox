use std::{ops::ControlFlow, process::ExitStatus, time::Duration};

use crossterm::event::MouseEventKind;
use miette::{Context as _, IntoDiagnostic, miette};
use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::Rect,
};
use tokio::{
    io::{self, AsyncReadExt as _, AsyncWriteExt as _},
    process,
    task::JoinHandle,
};

pub use self::pane::{AppPane, AppPaneId, AppPaneMessage};
use crate::{
    logger::{self, LogData, LogKind},
    tui,
};

mod pane;

pub enum AppMessage {
    Ping,
    Pane {
        pane_id: pane::AppPaneId,
        inner: pane::AppPaneMessage,
    },
}

impl AppMessage {
    pub const fn pane(pane_id: pane::AppPaneId, inner: pane::AppPaneMessage) -> Self {
        Self::Pane { pane_id, inner }
    }
}

pub type AppSender = tokio::sync::mpsc::Sender<AppMessage>;

pub struct App {
    panes: Vec<AppPane>,
    selected_pane: usize,
    quit: bool,
    ping_task: Option<JoinHandle<()>>,
}

impl App {
    /// Create a new app with a receiver.
    pub fn new(sender: AppSender) -> Self {
        let ping_task = tokio::spawn(Self::ping(sender));
        Self {
            ping_task: Some(ping_task),
            panes: Vec::new(),
            selected_pane: 0,
            quit: false,
        }
    }

    async fn ping(sender: AppSender) {
        loop {
            match sender.send(AppMessage::Ping).await {
                Ok(()) => {}
                Err(_) => break,
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    /// Add a new pane to the app.
    pub fn add_pane<F>(&mut self, f: F) -> pane::AppPaneId
    where
        F: FnOnce(AppPaneId) -> AppPane,
    {
        let id = pane::AppPaneId(self.panes.len());
        self.panes.push(f(id));
        id
    }

    fn pane_mut(&mut self, id: pane::AppPaneId) -> Option<&mut AppPane> {
        self.panes.get_mut(id.0)
    }

    fn pane_id_at(&self, column: u16, row: u16) -> Option<usize> {
        self.panes
            .iter()
            .enumerate()
            .find(|(_, pane)| pane.is_in_area(column, row))
            .map(|(i, _)| i)
    }

    fn split_even(length: u16, parts: u16) -> impl Iterator<Item = u16> {
        let base = length / parts;
        let remainder = length % parts;
        // Distribute the remainder one by one
        (0..parts).map(move |i| base + u16::from(i < remainder))
    }
}

impl tui::TuiApp for App {
    type Message = AppMessage;
    type Return = Vec<pane::ProgramHandle>;

    async fn handle_event(&mut self, event: Event) -> miette::Result<()> {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => {
                        self.quit = true;
                    }
                    KeyCode::Left => {
                        self.selected_pane = self.selected_pane.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        self.selected_pane = self
                            .selected_pane
                            .saturating_add(1)
                            .min(self.panes.len() - 1);
                    }
                    _ => {}
                }
                self.pane_mut(pane::AppPaneId(self.selected_pane))
                    .ok_or_else(|| miette!("pane not found"))?
                    .handle_event(&event);
            }
            Event::Mouse(mouse) => {
                if let Some(i) = self.pane_id_at(mouse.column, mouse.row) {
                    if let MouseEventKind::Down(_) = mouse.kind {
                        self.selected_pane = i;
                    }
                    self.pane_mut(pane::AppPaneId(i))
                        .ok_or_else(|| miette!("pane not found"))?
                        .handle_event(&event);
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: Self::Message) -> miette::Result<()> {
        match message {
            AppMessage::Pane { pane_id, inner } => {
                self.pane_mut(pane_id)
                    .ok_or_else(|| miette!("pane not found"))?
                    .handle_message(inner)
                    .await
                    .wrap_err("handle pane message")?;
            }
            AppMessage::Ping => {}
        }
        Ok(())
    }

    async fn update(&mut self) -> miette::Result<ControlFlow<Self::Return>> {
        if self.quit {
            if let Some(task) = self.ping_task.take() {
                task.abort();
                let _ = task.await;
            }
            let handles = self
                .panes
                .iter_mut()
                .filter_map(pane::AppPane::take_handle)
                .collect();
            return Ok(ControlFlow::Break(handles));
        }
        let futs = self.panes.iter_mut().map(pane::AppPane::update);
        futures::future::try_join_all(futs)
            .await
            .wrap_err("join pane updates")?;
        Ok(ControlFlow::Continue(()))
    }

    fn view(&mut self, frame: &mut Frame) -> miette::Result<()> {
        let area = frame.area();
        // Calculate pane count.
        let parts = self
            .panes
            .len()
            .try_into()
            .into_diagnostic()
            .wrap_err("convert pane count to u16")?;
        // Calculate pane widths.
        let pane_widths = || Self::split_even(area.width, parts);
        // Render panes horizontally.
        self.panes
            .iter_mut()
            .enumerate()
            .try_for_each(|(i, pane)| {
                let x = area.x + pane_widths().take(i).sum::<u16>();
                let width = pane_widths()
                    .nth(i)
                    .ok_or_else(|| miette!("pane not found"))?;
                pane.view(frame, Rect::new(x, area.y, width, area.height))
                    .wrap_err("pane view")
            })
            .wrap_err("render panes")?;
        Ok(())
    }
}

/// Prints the current working directory to a pane.
pub async fn print_cwd(sender: &AppSender, pane_id: AppPaneId) -> miette::Result<()> {
    let root_dir = crate::fs::cargo_workspace()
        .await
        .into_diagnostic()
        .wrap_err("find cargo workspace root")?;
    let msg = format!("Current working directory: {}", root_dir.display());
    send_info_message(sender, pane_id, msg)
        .await
        .wrap_err("send cwd message")?;
    Ok(())
}

/// Sends an info message to a pane.
pub async fn send_info_message(
    sender: &AppSender,
    pane_id: AppPaneId,
    message: impl AsRef<str>,
) -> miette::Result<()> {
    send_log_data(
        sender,
        pane_id,
        LogData::new(LogKind::XTask, format!("\n{}\n", message.as_ref())),
    )
    .await
    .wrap_err("send info message")?;
    Ok(())
}

/// Sends log data to a pane.
pub async fn send_log_data(
    sender: &AppSender,
    pane_id: AppPaneId,
    log_data: LogData,
) -> miette::Result<()> {
    sender
        .send(AppMessage::pane(pane_id, AppPaneMessage::LogData(log_data)))
        .await
        .into_diagnostic()
        .wrap_err("send log data")
}

/// Sends the finish message to the app.
pub async fn send_finish_message(
    sender: &AppSender,
    pane_id: AppPaneId,
    duration: Duration,
    exit_status: ExitStatus,
) -> miette::Result<()> {
    sender
        .send(AppMessage::pane(
            pane_id,
            AppPaneMessage::Finish {
                duration,
                exit_status,
            },
        ))
        .await
        .into_diagnostic()
        .wrap_err("send finish message")
}

/// Async task that forwards the output of a command to the logger.
pub async fn forward_output(
    sender: AppSender,
    pane_id: AppPaneId,
    output_name: impl AsRef<str>,
    stdout: process::ChildStdout,
    stderr: process::ChildStderr,
) -> miette::Result<()> {
    const BUF_SIZE: usize = 1024;

    // Create readers.
    let mut stdout_reader = io::BufReader::new(stdout);
    let mut stderr_reader = io::BufReader::new(stderr);
    // Create buffers.
    let mut stdout_buf = [0u8; BUF_SIZE];
    let mut stderr_buf = [0u8; BUF_SIZE];
    // Create output file.
    let path = format!(
        "target/{}/{}.log",
        env!("CARGO_PKG_NAME"),
        output_name.as_ref()
    );
    let output_file = crate::fs::open_clear_file(&path)
        .await
        .into_diagnostic()
        .wrap_err("create output file")?;
    // Create output file writer.
    let mut file_writer = io::BufWriter::new(output_file);
    send_info_message(&sender, pane_id, format!("Logging to: {path}",))
        .await
        .wrap_err("send info message")?;
    // Loop until both readers are done.
    let mut stdout_done = false;
    let mut stderr_done = false;
    loop {
        // Read stdout and stderr into buffers.
        let data = tokio::select!(
            result = stdout_reader
                .read(&mut stdout_buf), if !stdout_done => {
                    let read = result.into_diagnostic().wrap_err("read stdout")?;
                    if read == 0 {
                        stdout_done = true;
                        None
                    } else {
                        let buf = logger::normalize_newline(&mut stdout_buf[..read]);
                        Some(LogData::new(LogKind::Stdout, buf))
                    }
                }
            result = stderr_reader
                .read(&mut stderr_buf), if !stderr_done => {
                    let read = result.into_diagnostic().wrap_err("read stderr")?;
                    if read == 0 {
                        stderr_done = true;
                        None
                    } else {
                        let buf = logger::normalize_newline(&mut stderr_buf[..read]);
                        Some(LogData::new(LogKind::Stderr, buf))
                    }
                }
            // Break if both readers are done.
            else => break,
        );
        // Send data to logger.
        if let Some(data) = data {
            let file_buf = data.buf.clone();
            let write_file = async {
                file_writer
                    .write_all(&file_buf)
                    .await
                    .into_diagnostic()
                    .wrap_err("write stdout")
            };
            let send_log = async {
                send_log_data(&sender, pane_id, data)
                    .await
                    .wrap_err("send stdout")
            };
            tokio::try_join!(send_log, write_file).wrap_err("join send log and write file")?;
        }
        // Flush file writer.
        file_writer
            .flush()
            .await
            .into_diagnostic()
            .wrap_err("flush file writer")?;
    }
    Ok(())
}

/// Spawns [`forward_output`] as a task.
/// The returned future runs until output reaches EOF.
pub async fn spawn_forward_output(
    sender: AppSender,
    pane_id: AppPaneId,
    output_name: impl AsRef<str> + Send + 'static,
    stdout: process::ChildStdout,
    stderr: process::ChildStderr,
) -> miette::Result<()> {
    let forward_result = tokio::spawn(forward_output(sender, pane_id, output_name, stdout, stderr))
        .await
        .into_diagnostic()
        .wrap_err("join task output async task")?;
    forward_result.wrap_err("forward task output")
}
