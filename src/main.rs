// whack_a_mole - exploring async programming via the popular mole smashing pastime

// TODO - remove this!
// #![allow(warnings)]

mod game;
mod game_board;
mod game_display;
mod message_format;
mod mole;
mod renderer;
mod tracing_layer;
mod whacker;

use tokio::sync::mpsc;
use tracing_appender;
use tracing_subscriber::prelude::*;

use game::Game;
use renderer::Renderer;
use tracing_layer::TracingLayer;

const DISPLAY_CHANNEL_CAPACITY: usize = 32;

#[tokio::main]
// return type is an Error sink that can accept almost all errors from deeper in the code
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up async channels
    let (game_tx, renderer_rx) = mpsc::channel(DISPLAY_CHANNEL_CAPACITY);
    // for sending traces to the debug panel
    let trace_debug_tx = game_tx.clone();

    let game = Game::new(game_tx);
    let renderer = Renderer::new(renderer_rx);
    let tracing_layer = TracingLayer::new(trace_debug_tx);

    // construct a subscriber registry to allow for layers (top level, debug panel output)
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(tracing_appender::rolling::never("logs", "trace.log"));

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_layer)
        .init(); // make this the global default

    let game_task = tokio::spawn(game.run());
    let renderer_task = tokio::spawn(renderer.run());

    let _ = tokio::join!(game_task, renderer_task);

    Ok(())
}
