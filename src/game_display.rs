// GameDisplay - ratatui-based UI for whackamole
// TODO - add ratatui here

use crate::renderer;

const DEFAULT_BOARD_TITLE: &str = "Garden";
const DEFAULT_STATUS_TITLE: &str = "Journal";
const DEFAULT_DEBUG_TITLE: &str = "Debug";

#[derive(Debug)]
pub struct GameDisplay<'a> {
    board_title: &'a str,         // title of the game board portion of the display
    status_title: &'a str, // title of the status area of the display for updates (eg Mole Whacked!)
    debug_title: &'a str,  // optional debug window
    renderer: renderer::Renderer, // display renderer to handle panel updates & draws
}

impl<'a> Default for GameDisplay<'a> {
    fn default() -> Self {
        Self {
            board_title: DEFAULT_BOARD_TITLE,
            status_title: DEFAULT_STATUS_TITLE,
            debug_title: DEFAULT_DEBUG_TITLE,
            renderer: renderer::Renderer::new(), // renderer from top-level render module
        }
    }
}

impl<'a> GameDisplay<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn draw(&mut self) -> std::io::Result<()> {
        self.renderer.draw()?;
        Ok(())
    }
}
