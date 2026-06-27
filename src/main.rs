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

use ratatui;

use game::Game;
use game_display::GameDisplay;
use renderer::Renderer;

const DISPLAY_CHANNEL_CAPACITY: usize = 32;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // display panel - game
    let (display_game_tx, display_game_rx) = mpsc::channel(DISPLAY_CHANNEL_CAPACITY);

    let mut game = Game::new(display_game_tx);
    let mut renderer = Renderer::new(display_game_rx);

    let game_task = tokio::spawn(game.run());
    let renderer_task = tokio::spawn(renderer.run());

    let _ = tokio::join!(game_task, renderer_task);

    ratatui::restore();

    Ok(())
}
