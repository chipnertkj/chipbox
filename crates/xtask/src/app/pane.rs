use std::{
    pin::Pin,
    process::ExitStatus,
    time::{Duration, Instant},
};

use crossterm::event::{KeyCode, MouseEventKind};
use miette::{Context as _, IntoDiagnostic as _};
use ratatui::{
    Frame,
    crossterm::event::Event,
    layout::{Margin, Position, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Wrap,
    },
};
use tokio::task::JoinHandle;

use crate::logger::{self, Logger};

pub enum AppPaneMessage {
    Finish {
        duration: Duration,
        exit_status: ExitStatus,
    },
    LogData(logger::LogData),
}

#[derive(Clone, Copy)]
pub struct AppPaneId(pub(super) usize);

#[derive(Default)]
pub enum ProgramStatus {
    Running {
        handle: ProgramHandle,
        start: Instant,
    },
    #[default]
    Idle,
    Finished {
        exit_status: ExitStatus,
        duration: Duration,
    },
}

pub type ProgramHandle = JoinHandle<miette::Result<()>>;
type ProgramFuture = Pin<Box<dyn Future<Output = miette::Result<()>> + Send>>;
type ProgramFn = dyn Fn() -> ProgramFuture + Send;

pub struct AppPane {
    program_name: String,
    program_fn: Box<ProgramFn>,
    status: ProgramStatus,
    logger: Logger,
    scroll: Scroll,
}

#[derive(Default)]
struct Scroll {
    offset: u16,
    max_offset: u16,
    last_area: Rect,
}

impl Scroll {
    const LINES_PER_SCROLL_MOUSE: u16 = 3;
    const ADD_OFFSET: u16 = 1;

    fn wrap(&mut self) {
        self.offset = self.offset.min(self.max_offset);
    }

    fn up(&mut self, count: u16) {
        self.offset = self.offset.saturating_sub(count);
        self.wrap();
    }

    fn down(&mut self, count: u16) {
        self.offset = self.offset.saturating_add(count);
        self.wrap();
    }

    fn up_mouse(&mut self) {
        self.up(Self::LINES_PER_SCROLL_MOUSE);
    }

    fn down_mouse(&mut self) {
        self.down(Self::LINES_PER_SCROLL_MOUSE);
    }

    fn page_up(&mut self) {
        self.up(self.last_area.height);
    }

    fn page_down(&mut self) {
        self.down(self.last_area.height);
    }

    /// Update scroll state based on text height and area
    fn update_state(&mut self, text_height: u16, area_height: u16) {
        let max_offset = text_height
            .saturating_add(Self::ADD_OFFSET)
            .saturating_sub(area_height);

        // Auto-scroll to bottom if we were already at the bottom
        if self.max_offset == self.offset {
            self.offset = max_offset;
        }

        self.max_offset = max_offset;
        self.wrap();
    }

    /// Create scrollbar state for rendering
    fn create_scrollbar_state(&self) -> ScrollbarState {
        ScrollbarState::new(self.max_offset.into()).position(self.offset.into())
    }

    /// Create scrollbar widget with appropriate symbols
    fn create_scrollbar(&self) -> Scrollbar<'static> {
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol((self.offset > 0).then_some("↑"))
            .end_symbol((self.offset < self.max_offset).then_some("↓"))
    }

    /// Get current scroll offset for paragraph
    const fn scroll_offset(&self) -> (u16, u16) {
        (self.offset, 0)
    }

    /// Reset scroll state
    const fn reset(&mut self) {
        self.offset = 0;
        self.max_offset = 0;
    }
}

impl AppPane {
    pub fn take_handle(&mut self) -> Option<ProgramHandle> {
        let status = std::mem::take(&mut self.status);
        match status {
            ProgramStatus::Running { handle, .. } => Some(handle),
            _ => None,
        }
    }

    pub fn new<F, Fut>(program_name: impl Into<String>, program_fn: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = std::result::Result<(), miette::Report>> + Send + 'static,
    {
        const ENTRIES: usize = 8192;
        const PARTIAL_BYTES: usize = 16 * 1024usize.pow(2);
        let program_fn = Box::new(move || Box::pin(program_fn()) as ProgramFuture);
        let join_handle = tokio::spawn(program_fn());
        Self {
            program_name: program_name.into(),
            program_fn: Box::new(program_fn),
            status: ProgramStatus::Running {
                handle: join_handle,
                start: Instant::now(),
            },
            logger: Logger::new(ENTRIES, PARTIAL_BYTES),
            scroll: Scroll::default(),
        }
    }

    pub const fn is_in_area(&self, column: u16, row: u16) -> bool {
        self.scroll
            .last_area
            .contains(Position { x: column, y: row })
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => self.scroll.up(1),
                KeyCode::Down => self.scroll.down(1),
                KeyCode::PageUp => self.scroll.page_up(),
                KeyCode::PageDown => self.scroll.page_down(),
                KeyCode::Char('r') => {
                    self.restart_program();
                }
                _ => {}
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => self.scroll.up_mouse(),
                MouseEventKind::ScrollDown => self.scroll.down_mouse(),
                _ => {}
            },
            _ => {}
        }
    }

    fn restart_program(&mut self) {
        if let ProgramStatus::Running { .. } = self.status {
            return;
        }
        self.logger.clear();
        self.scroll.reset();
        self.status = ProgramStatus::Running {
            handle: tokio::spawn((self.program_fn)()),
            start: Instant::now(),
        };
    }

    pub async fn handle_message(&mut self, message: AppPaneMessage) -> miette::Result<()> {
        match message {
            AppPaneMessage::Finish {
                duration,
                exit_status,
            } => {
                if let ProgramStatus::Running { handle, .. } = &mut self.status {
                    handle
                        .await
                        .into_diagnostic()
                        .wrap_err("join program")?
                        .wrap_err("run program")?;
                }
                self.status = ProgramStatus::Finished {
                    duration,
                    exit_status,
                }
            }
            AppPaneMessage::LogData(log_data) => {
                self.logger.write_data(&log_data.buf, log_data.kind);
            }
        }
        Ok(())
    }

    pub async fn update(&mut self) -> miette::Result<()> {
        if let ProgramStatus::Running { handle, .. } = &mut self.status
            && handle.is_finished()
        {
            let result = handle.await;
            let () = result
                .into_diagnostic()
                .wrap_err("join program handle")?
                .wrap_err("run program")?;
            self.status = ProgramStatus::Idle;
        }
        Ok(())
    }

    pub fn view(&mut self, frame: &mut Frame, mut area: Rect) -> miette::Result<()> {
        // Update persistent state.
        self.scroll.last_area = area;
        // Split area into two to show status line.
        let status_area = area.rows().next_back().unwrap_or_default();
        let action_area = area.rows().nth_back(2).unwrap_or_default().inner(Margin {
            vertical: 0,
            horizontal: 1,
        });
        area.height = area
            .height
            .saturating_sub(action_area.height + status_area.height);
        // Paragraph.
        let text = Text::from(self.logger.to_lines());
        let output_block = Block::default()
            .title(self.program_name.as_ref())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1));
        let inner_area = output_block.inner(area);
        // Calculate wrapped text height.
        let text_height: u16 = Self::text_height_wrapped(&text, inner_area.width.into())
            .try_into()
            .into_diagnostic()
            .wrap_err("convert text height to u16")?;

        // Update scroll state
        self.scroll.update_state(text_height, inner_area.height);

        // Create scrollbar components
        let mut scrollbar_state = self.scroll.create_scrollbar_state();
        let scrollbar = self.scroll.create_scrollbar();

        // Paragraph.
        let paragraph = Paragraph::new(text)
            .block(output_block)
            .wrap(Wrap { trim: false })
            .scroll(self.scroll.scroll_offset());
        // Status line.
        let status = match &self.status {
            ProgramStatus::Running { start, .. } => {
                let duration =
                    humantime::format_duration(Duration::from_secs(start.elapsed().as_secs()));
                let msg = format!("Running for {duration}...");
                Some((Color::DarkGray, msg))
            }
            ProgramStatus::Idle => None,
            ProgramStatus::Finished {
                exit_status,
                duration,
            } => {
                let duration = humantime::format_duration(Duration::from_secs(duration.as_secs()));
                let msg = format!(
                    "{} in {duration} with {exit_status}",
                    if exit_status.success() {
                        "Finished"
                    } else {
                        "Failed"
                    }
                );
                let color = if exit_status.success() {
                    Color::LightGreen
                } else {
                    Color::LightRed
                };
                Some((color, msg))
            }
        };
        let action_spans = |key: &'static str, action: &'static str| {
            [
                Span::styled("[", Style::default().fg(Color::DarkGray)),
                Span::styled(key, Style::default().fg(Color::Magenta)),
                Span::styled("]", Style::default().fg(Color::DarkGray)),
                Span::styled(": ", Style::default().fg(Color::DarkGray)),
                Span::styled(action, Style::default().fg(Color::LightCyan)),
            ]
            .into_iter()
        };
        let status_spans = status.map_or_else(
            || Box::new([].into_iter()) as Box<dyn Iterator<Item = Span<'_>>>,
            |(color, msg)| Box::new([Span::styled(msg, Style::default().fg(color))].into_iter()),
        );
        let finish_spans: Box<dyn Iterator<Item = Span<'_>>> =
            if let ProgramStatus::Finished { .. } = &self.status {
                Box::new(std::iter::once(" | ".into()).chain(action_spans("r", "restart")))
            } else {
                Box::new([].into_iter())
            };
        let status_line = status_spans.collect::<Line>();
        let action_line = action_spans("q", "quit")
            .chain(finish_spans)
            .collect::<Line>();
        // Render content.
        frame.render_widget(paragraph, area);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
        frame.render_widget(action_line, action_area);
        frame.render_widget(status_line, status_area);
        Ok(())
    }

    fn text_height_wrapped(text: &Text, width: usize) -> usize {
        textwrap::wrap(&text.to_string(), width).len()
    }
}
