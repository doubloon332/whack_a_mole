// module to handle ratatui message passing for UI layout, content updates, and screen draws

use crate::message_format::{DisplayTextUpdate};

use tokio::sync::mpsc;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{
    Frame,
    widgets::{Block, BorderType, Borders, Paragraph},
};

// percentages to define all 3 panels (game, status, board)
const TOP_PANEL_PERC: u16 = 70;
const TOP_LEFT_PANEL_PERC: u16 = 60;

#[derive(Debug)]
pub struct Renderer {
    terminal: ratatui::DefaultTerminal, // terminal object with Backend generic
    game_display_rx: mpsc::Receiver<DisplayTextUpdate>,
    screen: Screen,
}

#[derive(Debug)]
pub enum Panels {
    GamePanel,
    StatusPanel,
    DebugPanel,
}

#[derive(Debug)]
struct Panel {
    panel_type: Panels,
    title: String,
    text: String,
}

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
    panels: Vec<self::Panel>,
}

impl Screen {
    fn new() -> Self {
        Default::default()
    }
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
                    panel_type: Panels::GamePanel,
                    title: String::from("Garden"),
                    text: String::from(""),
                },
                Panel {
                    panel_type: Panels::StatusPanel,
                    title: String::from("Journal"),
                    text: String::from(""),
                },
                Panel {
                    panel_type: Panels::DebugPanel,
                    title: String::from("Debug"),
                    text: String::from(""),
                },
            ],
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ratatui::restore();
    }
}   

impl Renderer {
    // display setup - clear screen & set terminal
    pub fn new(game_rx: mpsc::Receiver<DisplayTextUpdate>) -> Self {
        Renderer {
            terminal: ratatui::init(),
            game_display_rx: game_rx,
            screen: Screen::new(),
        }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        self.draw();
        self.recv_game_updates().await;
        Ok(())
    }

    // get updates from Game channel & apply to display
    async fn recv_game_updates(&mut self) {
        while let Some(msg) = self.game_display_rx.recv().await {
            match msg {
                DisplayTextUpdate::Text(text) => {
                    self.screen.panels[0].text.push_str(&text);
                    let _ = self.draw();
                },
                DisplayTextUpdate::Exit => {
                    break;
                },
            }
        }
    }

    // draw the current screen as supplied by render()
    pub fn draw(&mut self) -> std::io::Result<()> {
        self.terminal.draw(|frame| render(frame, &self.screen));
        Ok(())
    }

    // display teardown
    pub fn restore(self) {
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
        match &panel.panel_type {
            Panels::GamePanel => {
                // render board
                frame.render_widget(
                    Paragraph::new(panel.text.clone()).block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title(panel.title.clone())
                            .border_type(BorderType::Rounded),
                    ),
                    game_layout[0],
                );
            },
            Panels::StatusPanel => {
                // render status
                frame.render_widget(
                     Paragraph::new(panel.text.clone()).block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title(panel.title.clone())
                            .border_type(BorderType::Rounded),
                    ),
                    game_layout[1],
                );
            },
            Panels::DebugPanel => {
                    // render debug
                    frame.render_widget(
                    Paragraph::new(panel.text.clone()).block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title(panel.title.clone())
                            .border_type(BorderType::Rounded),
                    ),
                    screen_layout[1],
                );
            },
        }
    }
}
