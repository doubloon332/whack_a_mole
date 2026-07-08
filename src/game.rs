// Game - manages the game state, i.e. moles, whacker, gameboard, run loop, etc

use crate::message_format::RenderMessage;
use crate::mole::Mole;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures_util::StreamExt;
use tokio::sync::{mpsc, watch};

use tracing;

use chrono::Local;
use std::io;

const GAME_DEFAULT_NUM_MOLES: u32 = 4;

#[derive(Debug)]
pub struct Game {
    exit: bool,
    pub num_moles: u32,
    moles: Vec<Mole>,
    render_tx: mpsc::Sender<RenderMessage>,
    shutdown_tx: watch::Sender<bool>,
    panel_update_buffer: Vec<RenderMessage>,
}

impl Game {
    // create a new game
    pub fn new(game_tx: mpsc::Sender<RenderMessage>, quit_tx: watch::Sender<bool>) -> Self {
        Self {
            exit: false,
            num_moles: GAME_DEFAULT_NUM_MOLES,
            moles: init_moles(GAME_DEFAULT_NUM_MOLES),
            render_tx: game_tx,
            shutdown_tx: quit_tx,
            panel_update_buffer: vec![],
        }
    }

    pub async fn run(mut self) -> Result<(), io::Error> {
        tracing::info!("{}: Entered Game::run()", Local::now());

        // create an event stream to handle keypresses
        let mut events = EventStream::new();
        while !self.exit {
            self.update_game_display().await?;

            match events.next().await {
                // awaits cooperatively, no thread block
                Some(Ok(event)) => self.handle_event(event),
                Some(Err(e)) => return Err(e),
                None => break, // stream ended
            }
        }
        Ok(())
    }

    pub async fn update_game_display(&mut self) -> Result<(), io::Error> {
        for item in self.panel_update_buffer.drain(..) {
            if self.render_tx.send(item).await.is_err() {
                panic!("Render receiver dropped, quitting!")
            };
        }
        Ok(())
    }

    // handle keypresses & any other events (from ratatui example code)
    fn handle_event(&mut self, event: Event) {
        match event {
            // check that the event is a key press event as crossterm also emits
            // key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
    }

    // handle keypresses (from ratatui example code)
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.shutdown(),
            _ => {}
        }
    }

    fn shutdown(&mut self) {
        self.exit = true;
        let _ = self.shutdown_tx.send(true);
    }
}

// generate moles with default values
fn init_moles(num_moles: u32) -> Vec<Mole> {
    tracing::info!("{}: Generating {} moles", Local::now(), num_moles);
    (0..num_moles).map(|_| Mole::new()).collect()
}
