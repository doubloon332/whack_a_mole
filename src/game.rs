// Game - manages the game state, i.e. moles, whacker, gameboard, run loop, etc

use crate::game_display::GameDisplay;
use crate::message_format::DisplayTextUpdate;
use crate::mole::Mole;
use crate::renderer::Panels;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use tokio::sync::mpsc;

use std::io;

const GAME_DEFAULT_NUM_MOLES: u32 = 4;
// capacity of the mpsc channels used to update the display
const DISPLAY_CHANNEL_CAPACITY: usize = 32;

#[derive(Debug)]
pub struct Game<'a> {
    exit: bool,
    num_moles: u32,
    moles: Vec<Mole<'a>>,
    display: GameDisplay<'a>,
    channel_buffer: ChannelBuffer,
}

#[derive(Debug)]
struct ChannelBuffer {
    game_buffer: Vec<String>,
}

impl<'a> Game<'a> {
    // create a new game
    pub fn new() -> Self {
        Self {
            exit: false,
            num_moles: GAME_DEFAULT_NUM_MOLES,
            moles: init_moles(GAME_DEFAULT_NUM_MOLES),
            display: GameDisplay::new(),
            channel_buffer: ChannelBuffer {
                game_buffer: vec![],
            },
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        while !self.exit {
            // self.display.draw()?;
            self.handle_events()?;
            todo!();
        }

        Ok(())
    }

    async fn spawn_display_channels(&mut self) {
        let _ = self.spawn_game_channel();
    }

    // channel for updating game display panel
    async fn spawn_game_channel(&self) {
        let (tx, rx) = mpsc::channel(DISPLAY_CHANNEL_CAPACITY);

        let sender_task = tokio::spawn(self.run_game_text_send(tx));
        let receiver_task = tokio::spawn(self.display.renderer.run_game_text_recv(rx));

        let _ = tokio::join!(sender_task, receiver_task);
    }

    async fn run_game_text_send(&mut self, tx: mpsc::Sender<DisplayTextUpdate>) -> io::Result<()> {
        while !(self.exit) {
            if self.channel_buffer.game_buffer.len() > 0 {
                for item in &self.channel_buffer.game_buffer {
                    let msg = DisplayTextUpdate::Text(item.to_string());
                    if tx.send(msg).await.is_err() {
                        eprintln!("Game display receiver dropped, stopping");
                        break;
                    }
                }
                self.channel_buffer.game_buffer.clear();
            }
        }

        Ok(())
    }

    // handle keypresses & any other events (from ratatui example code)
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // check that the event is a key press event as crossterm also emits
            // key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
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
fn init_moles<'a>(num_moles: u32) -> Vec<Mole<'a>> {
    let mut moles = vec![];

    for _ in 0..num_moles {
        let this_mole = Mole::new();
        moles.push(this_mole);
    }

    moles
}
