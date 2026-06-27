// GameDisplay - ratatui-based UI for whackamole
// TODO - add ratatui here

const DEFAULT_BOARD_TITLE: &str = "Garden";
const DEFAULT_STATUS_TITLE: &str = "Journal";
const DEFAULT_DEBUG_TITLE: &str = "Debug";

#[derive(Debug)]
pub struct GameDisplay {
    board_title: String,  // title of the game board portion of the display
    status_title: String, // title of the status area of the display for updates (eg Mole Whacked!)
    debug_title: String,  // optional debug window
}

impl Default for GameDisplay {
    fn default() -> Self {
        Self {
            board_title: String::from(DEFAULT_BOARD_TITLE),
            status_title: String::from(DEFAULT_STATUS_TITLE),
            debug_title: String::from(DEFAULT_DEBUG_TITLE),
        }
    }
}

impl GameDisplay {
    pub fn new() -> Self {
        Default::default()
    }
}
