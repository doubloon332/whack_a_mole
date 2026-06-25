// whack_a_mole - exploring async programming via the popular mole smashing pastime

mod game;
mod game_board;
mod game_display;
mod mole;
mod renderer;
mod renderer_draw;
mod whacker;

use game::Game;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut whackamole = Game::new();

    whackamole.init()?;
    whackamole.run()?;
    whackamole.quit()?;

    Ok(())
}
