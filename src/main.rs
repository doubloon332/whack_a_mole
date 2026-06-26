// whack_a_mole - exploring async programming via the popular mole smashing pastime

mod game;
mod game_board;
mod game_display;
mod message_format;
mod mole;
mod renderer;
mod whacker;

use tokio::sync::mpsc;
use tokio::task;

use game::Game;

// enum PanelMessage {
//     Message { panel_id: &str, val: &str },
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut whackamole = Game::new();
    whackamole.run()?;
    Ok(())
}
