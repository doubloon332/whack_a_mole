// module to handle ratatui message passing for UI layout, content updates, and screen draws

use crate::game_display::GameDisplay;
use crate::message_format::{
    BoardSnapshot, GamePanelOp, PanelContent, PanelKind, PanelOps, RenderMessage, TextPanelOp,
};

use tokio::sync::{mpsc, watch};
use tokio::time;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::canvas::Canvas;
use ratatui::{
    Frame,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use std::time::Duration;

// a little less than 60fps
const RENDER_TICK_DURATION_IN_MILLIS: u64 = 17;

// percentages to define all 3 panels (game, status, board)
const TOP_PANEL_PERC: u16 = 70;
const TOP_LEFT_PANEL_PERC: u16 = 60;

#[derive(Debug)]
// screen layout
struct Screen {
    // ratatui constructs 3 panels across 2 layouts:
    // first, split top & bottom portions (1 layout),
    // then split top left & top right (2nd layout)
    top_panel_percent: u16,
    bottom_panel_percent: u16,
    top_left_panel_percent: u16,
    top_right_panel_percent: u16,
    panels: Vec<Panel>,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            top_panel_percent: TOP_PANEL_PERC,
            bottom_panel_percent: 100 - TOP_PANEL_PERC,
            top_left_panel_percent: TOP_LEFT_PANEL_PERC,
            top_right_panel_percent: 100 - TOP_LEFT_PANEL_PERC,
            panels: vec![
                Panel {
                    panel_kind: PanelKind::Game,
                    title: String::from("Garden"),
                    content: PanelContent::Board(BoardSnapshot { moles: vec![] }),
                    border_color: Color::Green,
                },
                Panel {
                    panel_kind: PanelKind::Status,
                    title: String::from("Journal"),
                    content: PanelContent::Text(String::from("")),
                    border_color: Color::Magenta,
                },
                Panel {
                    panel_kind: PanelKind::Debug,
                    title: String::from("Debug"),
                    content: PanelContent::Text(String::from("")),
                    border_color: Color::Cyan,
                },
            ],
        }
    }
}

impl Screen {
    fn new() -> Self {
        Self::default()
    }

    // accessor for getting a certain panel for editing
    fn panel_mut(&mut self, kind: PanelKind) -> Option<&mut Panel> {
        self.panels.iter_mut().find(|p| p.panel_kind == kind)
    }
}

#[derive(Debug)]
pub struct Panel {
    pub panel_kind: PanelKind,
    pub title: String,
    pub content: PanelContent,
    pub border_color: ratatui::style::Color, // from ratatui::Color
}

#[derive(Debug)]
pub struct Renderer {
    terminal: ratatui::DefaultTerminal, // terminal object with Backend generic
    render_rx: mpsc::Receiver<RenderMessage>,
    trace_rx: mpsc::UnboundedReceiver<RenderMessage>,
    shutdown_rx: watch::Receiver<bool>,
    tick_duration: u64,
    screen: Screen,
}
impl Renderer {
    // display setup - clear screen & set terminal
    pub fn new(
        game_rx: mpsc::Receiver<RenderMessage>,
        deb_rx: mpsc::UnboundedReceiver<RenderMessage>,
        quit_rx: watch::Receiver<bool>,
    ) -> Self {
        Renderer {
            terminal: ratatui::init(),
            render_rx: game_rx,
            trace_rx: deb_rx,
            shutdown_rx: quit_rx,
            tick_duration: RENDER_TICK_DURATION_IN_MILLIS,
            screen: Screen::new(),
        }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        self.draw()?;
        self.recv_panel_updates().await?;
        Ok(())
    }

    // get updates from Game channel & apply to display
    async fn recv_panel_updates(&mut self) -> std::io::Result<()> {
        let mut interval = time::interval(Duration::from_millis(self.tick_duration));

        tracing::info!(
            "Entered Renderer::recv_panel_updates() with tick interval {}ms",
            self.tick_duration
        );

        // redraw every tick, wait on messages received or shutdown signal
        loop {
            tokio::select! {
                _ = interval.tick() => self.draw()?,
                _ = self.shutdown_rx.changed() => break,
                Some(msg) = self.render_rx.recv() => { self.apply(msg) }
                Some(msg) = self.trace_rx.recv() => { self.apply(msg) }
            }

            // drain everything else in both queues
            while let Ok(msg) = self.render_rx.try_recv() {
                self.apply(msg);
            }
            while let Ok(msg) = self.trace_rx.try_recv() {
                self.apply(msg);
            }

            // draw screen
            self.draw()?;
        }
        Ok(())
    }

    // apply the contents of a message to the screen (but don't draw)
    fn apply(&mut self, msg: RenderMessage) {
        match msg {
            RenderMessage::Panel(update) => {
                if let Some(panel) = self.screen.panel_mut(update.target) {
                    match update.op {
                        PanelOps::Game(game_op) => self.update_board(game_op),
                        PanelOps::Text(text_op) => {
                            if let PanelContent::Text(s) = &mut panel.content {
                                match text_op {
                                    TextPanelOp::Append(t) => {
                                        s.push_str(&t);
                                    }
                                    TextPanelOp::Replace(t) => {
                                        s.replace_range(.., &t);
                                    }
                                    TextPanelOp::Clear => {
                                        s.clear();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // update or clear the game board panel (but don't draw)
    fn update_board(&mut self, op: GamePanelOp) {
        todo!();
    }

    // draw the current screen as supplied by render()
    pub fn draw(&mut self) -> std::io::Result<()> {
        self.terminal.draw(|frame| render(frame, &self.screen))?;
        Ok(())
    }
}
// terminal teardown
impl Drop for Renderer {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

// draw a frame to the screen
fn render(frame: &mut Frame, screen: &Screen) {
    // slice screen
    let screen_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(screen.top_panel_percent),
            Constraint::Percentage(screen.bottom_panel_percent),
        ])
        .split(frame.area());

    // split top pane into game areas (Garden & Journal)
    let game_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(screen.top_left_panel_percent),
            Constraint::Percentage(screen.top_right_panel_percent),
        ])
        .split(screen_layout[0]);

    for panel in &screen.panels {
        let area = match panel.panel_kind {
            PanelKind::Game => game_layout[0],
            PanelKind::Status => game_layout[1],
            PanelKind::Debug => screen_layout[1],
        };

        match &panel.content {
            PanelContent::Text(s) => {
                // the Debug panel follows its tail, the others render from the top (for now)
                let scroll_y = match panel.panel_kind {
                    PanelKind::Debug | PanelKind::Status => {
                        let total_lines = s.lines().count() as u16;
                        let inner_height = area.height.saturating_sub(2);
                        total_lines.saturating_sub(inner_height)
                    }
                    PanelKind::Game => 0,
                };

                // create Debug & Status Paragraph widgets
                let widget = Paragraph::new(s.as_str()).scroll((scroll_y, 0)).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(panel.border_color))
                        .title(panel.title.as_str())
                        .border_type(BorderType::Rounded),
                );
                frame.render_widget(widget, area);
            }
            PanelContent::Board(snapshot) => {
                // draw borders & title
                let border = Block::new()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(panel.border_color))
                    .title(panel.title.as_str())
                    .border_type(BorderType::Rounded);
                frame.render_widget(border, area);

                // draw the game board using GameDisplay
                let game_display = GameDisplay::new(&snapshot);
                game_display.render(frame, area);
            }
        }
    }
}
