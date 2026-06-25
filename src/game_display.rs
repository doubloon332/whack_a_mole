// GameDisplay - ratatui-based UI for whackamole
// TODO - add ratatui here

const DEFAULT_BOARD_TITLE: &str = "Garden";
const DEFAULT_STATUS_TITLE: &str = "Journal";
const DEFAULT_DEBUG_TITLE: &str = "Debug";

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    symbols::border,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

#[derive(Debug)]
pub struct GameDisplay<'a> {
    board_title: &'a str,  // title of the game board portion of the display
    status_title: &'a str, // title of the status area of the display for updates (eg Mole Whacked!)
    debug_title: &'a str,  // optional debug window
    terminal: ratatui::DefaultTerminal, // terminal object with Backend generic
}

impl<'a> Default for GameDisplay<'a> {
    fn default() -> Self {
        Self {
            board_title: DEFAULT_BOARD_TITLE,
            status_title: DEFAULT_STATUS_TITLE,
            debug_title: DEFAULT_DEBUG_TITLE,
            terminal: ratatui::init(),
        }
    }
}

impl<'a> GameDisplay<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    // display setup - clear screen & set terminal
    pub fn init(&mut self) {
        self.terminal = ratatui::init();
    }

    // draw the current screen as supplied by render()
    pub fn draw(&mut self) -> std::io::Result<()> {
        self.terminal.draw(Self::render)?;
        Ok(())
    }

    // display teardown
    pub fn restore(&self) {
        ratatui::restore();
    }

    // build the current screen
    fn render(frame: &mut Frame) {
        // screen layout:
        // game area is top 70% is garden (mole area) & journal (status updates) arranged horizontally
        // game area split horizontally 60% garden & 40% journal
        // bottom 30% is debug output (spans entire screen)
        // TODO: modularize this with function calls, make this configurable with defaults

        // split into top & bottom (70%/30%) for game windows (Garden, Journal) and debug
        let screen_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(frame.area());

        // split top pane into game areas (Garden & Journal)
        let game_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(screen_layout[0]);

        // render Garden
        frame.render_widget(
            Paragraph::new("Garden TK").block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Garden")
                    .border_type(BorderType::Rounded),
            ),
            game_layout[0],
        );

        // render Journal
        frame.render_widget(
            Paragraph::new("Journal TK").block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Journal")
                    .border_type(BorderType::Rounded),
            ),
            game_layout[1],
        );

        // render Debug
        frame.render_widget(
            Paragraph::new("Debug TK").block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Debug")
                    .border_type(BorderType::Rounded),
            ),
            screen_layout[1],
        );

        //
    }
}
