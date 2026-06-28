// module to handle ratatui message passing for UI layout, content updates, and screen draws

use crate::message_format::{PanelKind, PanelOp, RenderMessage};

use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::{
    Frame,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use tracing::{Level, debug, event};

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
                    text: String::from(""),
                    border_color: Color::Green,
                },
                Panel {
                    panel_kind: PanelKind::Status,
                    title: String::from("Journal"),
                    text: String::from(""),
                    border_color: Color::Magenta,
                },
                Panel {
                    panel_kind: PanelKind::Debug,
                    title: String::from("Debug"),
                    text: String::from(""),
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

    // accessor for getting a certain panel
    fn panel_mut(&mut self, kind: PanelKind) -> Option<&mut Panel> {
        self.panels.iter_mut().find(|p| p.panel_kind == kind)
    }
}

#[derive(Debug)]
pub struct Panel {
    pub panel_kind: PanelKind,
    pub title: String,
    pub text: String,
    pub border_color: ratatui::style::Color, // from ratatui::Color
}

#[derive(Debug)]
pub struct Renderer {
    terminal: ratatui::DefaultTerminal, // terminal object with Backend generic
    render_rx: mpsc::Receiver<RenderMessage>,
    screen: Screen,
}
impl Renderer {
    // display setup - clear screen & set terminal
    pub fn new(game_rx: mpsc::Receiver<RenderMessage>) -> Self {
        Renderer {
            terminal: ratatui::init(),
            render_rx: game_rx,
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
        // wait until the first message of a burst & park it
        while let Some(first) = self.render_rx.recv().await {
            event!(Level::INFO, "Applying first message\n");
            debug!("Applying first message {:#?}\n", &first);
            let mut exit = self.apply(first);

            // drain rest of queue
            loop {
                match self.render_rx.try_recv() {
                    Ok(msg) => exit |= self.apply(msg),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        exit = true;
                        break;
                    }
                }
            }
            self.draw()?;
            if exit {
                break;
            };
        }
        Ok(())
    }

    // apply the contents of a message to the screen (but don't draw)
    fn apply(&mut self, msg: RenderMessage) -> bool {
        match msg {
            RenderMessage::Shutdown => return true,
            RenderMessage::Panel(update) => {
                if let Some(panel) = self.screen.panel_mut(update.target) {
                    match update.op {
                        PanelOp::Append(t) => panel.text.push_str(&t),
                        PanelOp::Replace(t) => panel.text = t,
                        PanelOp::Clear => panel.text.clear(),
                    }
                }
            }
        }
        false
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

        let widget = Paragraph::new(panel.text.as_str()).block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(panel.border_color))
                .title(panel.title.as_str())
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(widget, area);
    }
}
