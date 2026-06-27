// Game - manages the game state, i.e. moles, whacker, gameboard, run loop, etc


use crate::message_format::DisplayTextUpdate;
use crate::mole::Mole;

use crossterm::event::{self, Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures_util::StreamExt;
use tokio::sync::mpsc;

use std::io;

const GAME_DEFAULT_NUM_MOLES: u32 = 4;
// capacity of the mpsc channels used to update the display

#[derive(Debug)]
pub struct Game {
    exit: bool,
    num_moles: u32,
    moles: Vec<Mole>,
    game_display_tx: mpsc::Sender<DisplayTextUpdate>,
    pub channel_buffer: ChannelBuffer,
}

#[derive(Debug)]
struct ChannelBuffer {
    game_buffer: Vec<DisplayTextUpdate>,
}

impl Game {
    // create a new game
    pub fn new(game_tx: mpsc::Sender<DisplayTextUpdate>) -> Self {
        Self {
            exit: false,
            num_moles: GAME_DEFAULT_NUM_MOLES,
            moles: init_moles(GAME_DEFAULT_NUM_MOLES),
            game_display_tx: game_tx,
            channel_buffer: ChannelBuffer {
                game_buffer: vec![],
            },
        }
    }

    pub async fn run(mut self) -> Result<(), std::io::Error> {
        for _ in 1..100 {
            self.channel_buffer.game_buffer.push(DisplayTextUpdate::Text(String::from("Hello\n")));
            self.channel_buffer.game_buffer.push(DisplayTextUpdate::Text(String::from("yas bish\n")));
        }
        
        // create an event stream to handle keypresses
        let mut events = EventStream::new();
        while !self.exit {
            self.update_game_display().await;

            match events.next().await {          // awaits cooperatively, no thread block
                Some(Ok(event)) => self.handle_event(event),
                Some(Err(e))    => eprintln!("input error: {e}"),
                None            => break,         // stream ended
            }
        }

        Ok(())
    }

    pub async fn update_game_display(&mut self) -> io::Result<()> {
        if self.channel_buffer.game_buffer.len() > 0 {
            for item in self.channel_buffer.game_buffer.drain(..) {
                if self.game_display_tx.send(item).await.is_err() {
                    eprintln!("Game display receiver dropped, stopping");
                    break;
                }
            }
        }
        Ok(())
    }

    // handle keypresses & any other events (from ratatui example code)
    fn handle_event(&mut self, event:Event) {
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
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}

// generate moles with default values
fn init_moles<'a>(num_moles: u32) -> Vec<Mole> {
    let mut moles = vec![];

    for _ in 0..num_moles {
        let this_mole = Mole::new();
        moles.push(this_mole);
    }

    moles
}
