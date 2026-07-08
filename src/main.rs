// whack_a_mole - exploring async programming via the popular mole smashing pastime

mod game;
mod game_board;
mod game_display;
mod message_format;
mod mole;
mod renderer;
mod tracing_layer;
mod whacker;

use tokio::sync::{mpsc, watch};
use tracing_appender;
use tracing_subscriber::prelude::*;

use chrono::Local;

use game::Game;
use renderer::Renderer;
use tracing_layer::TracingLayer;

const DISPLAY_CHANNEL_CAPACITY: usize = 32;
const LOG_PATH: &str = "logs/";
const LOG_FILENAME: &str = "mole_trace.log";

#[tokio::main]
// return type is an Error sink that can accept almost all errors from deeper in the code
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up async channels
    let (game_tx, renderer_rx) = mpsc::channel(DISPLAY_CHANNEL_CAPACITY);
    // for sending traces to the debug panel
    let (trace_debug_tx, trace_debug_rx) = mpsc::unbounded_channel();
    // dedicated channel for signaling shutdown
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let game = Game::new(game_tx, shutdown_tx);
    let renderer = Renderer::new(renderer_rx, trace_debug_rx, shutdown_rx);
    let tracing_layer = TracingLayer::new(trace_debug_tx);

    // construct a subscriber registry to allow for layers (top level, debug panel output)
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(tracing_appender::rolling::never(LOG_PATH, LOG_FILENAME));

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_layer)
        .init(); // make this the global default

    tracing::info!("*********** New Whack A Mole game run started ***********",);

    tracing::info!(
        "Started tracing with logging to {}",
        [LOG_PATH, LOG_FILENAME].concat()
    );
    tracing::info!("Initialized game with {} moles", game.num_moles);

    let game_task = tokio::spawn(game.run());
    tracing::info!("Spawned game task");
    let renderer_task = tokio::spawn(renderer.run());
    tracing::info!("Spawned renderer task");

    let (game_res, renderer_res) = tokio::join!(game_task, renderer_task);

    if let Err(e) = game_res {
        tracing::error!("Tokio task for Game failed: {e:?}");
    }
    if let Err(e) = renderer_res {
        tracing::error!("Tokio task for Renderer failed: {e:?}");
    }

    Ok(())
}
