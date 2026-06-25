// Game - manages the game state, i.e. moles, whacker, gameboard, run loop, etc

use crate::game_display::GameDisplay;
use crate::mole::Mole;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use std::io;

const GAME_DEFAULT_NUM_MOLES: u32 = 4;

// top-level struct to run the game
#[derive(Debug)]
pub struct Game<'a> {
    exit: bool,
    num_moles: u32,
    moles: Vec<Mole<'a>>,
    display: GameDisplay<'a>,
}

impl<'a> Default for Game<'a> {
    fn default() -> Self {
        Self {
            exit: false,
            num_moles: GAME_DEFAULT_NUM_MOLES,
            moles: init_moles(GAME_DEFAULT_NUM_MOLES),
            display: GameDisplay::new(),
        }
    }
}

// method definitions
impl<'a> Game<'a> {
    // create a new game
    pub fn new() -> Self {
        Default::default()
    }

    // initialize game
    pub fn init(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        while !self.exit {
            self.display.draw()?;
            self.handle_events()?;
            todo!();
        }

        Ok(())
    }

    // shut down game
    pub fn quit(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    // handle keypresses & any other events (from ratatui example code)
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
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
