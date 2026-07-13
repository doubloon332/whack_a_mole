// Game - manages the game state, i.e. moles, whacker, gameboard, run loop, etc

use crate::hole::Hole;
use crate::message_format::RenderMessage;
use crate::mole::Mole;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};

use futures_util::StreamExt;
use tokio::sync::{mpsc, watch};
use tokio::time;

use tracing;

use std::fs::File;
use std::io;
use std::io::{BufReader, prelude::*};
use std::time::Duration;

use rand::random_range;
use rand::seq::IteratorRandom;

// game clock interval
const GAME_TICK_DURATION_IN_MILLIS: u64 = 50;

// row of 3, row of 2, row of 3, like the classic
// by default 1 mole per hole, but could change
//  (if moles travel between holes eventually)
const GAME_DEFAULT_NUM_MOLES: u64 = 8;
const GAME_DEFAULT_NUM_HOLES: u64 = 8;
const MIN_MOLES: u64 = 5;
const MAX_MOLES: u64 = 128;
const MIN_HOLES: u64 = 5;
const MAX_HOLES: u64 = 128;

const MOLE_NAME_FILE: &str = "./mole_names.txt";

#[derive(Debug)]
pub struct Game {
    exit: bool,
    tick_duration: u64,
    pub num_moles: u64,
    pub num_holes: u64,
    moles: Vec<Mole>,
    mole_names: Vec<String>,
    holes: Vec<Hole>,
    render_tx: mpsc::Sender<RenderMessage>,
    shutdown_tx: watch::Sender<bool>,
    panel_update_buffer: Vec<RenderMessage>,
}

impl Game {
    // create a new game
    pub fn new(game_tx: mpsc::Sender<RenderMessage>, quit_tx: watch::Sender<bool>) -> Self {
        Self {
            exit: false,
            tick_duration: GAME_TICK_DURATION_IN_MILLIS,
            num_moles: GAME_DEFAULT_NUM_MOLES,
            num_holes: GAME_DEFAULT_NUM_HOLES,
            moles: init_moles(GAME_DEFAULT_NUM_MOLES),
            mole_names: read_mole_names(MOLE_NAME_FILE),
            holes: init_holes(GAME_DEFAULT_NUM_HOLES),
            render_tx: game_tx,
            shutdown_tx: quit_tx,
            panel_update_buffer: vec![],
        }
    }

    pub fn init(&mut self) {
        tracing::info!(
            "Init: Naming {} moles from pool of {} names and {} holes",
            self.num_moles,
            self.mole_names.len(),
            self.num_holes
        );
        self.name_and_assign_moles();
    }

    pub async fn run(mut self) -> Result<(), io::Error> {
        let mut interval = time::interval(Duration::from_millis(self.tick_duration));

        tracing::info!(
            "Entered Game::run() with tick interval {}ms",
            self.tick_duration
        );

        // create an event stream to handle keypresses
        let mut events = EventStream::new();

        // act on game tick or keypress, whichever comes first
        while !self.exit {
            tokio::select! {
                _ = interval.tick() => self.update_game_display().await?,
                maybe_event = events.next() => {
                    match maybe_event {
                        Some(Ok(event)) => self.handle_event(event),
                        Some(Err(e)) => return Err(e),
                        None => break, // stream ended
                    }
                }
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
            KeyCode::Char('[') => {
                if self.num_moles > MIN_MOLES {
                    tracing::info!(
                        "Decreasing number of moles from {} to {}",
                        self.num_moles,
                        self.num_moles - 1
                    );
                    self.num_moles -= 1;
                    self.init();
                    tracing::info!(
                        "Decreasing number of holes from {} to {}",
                        self.num_holes,
                        self.num_holes - 1
                    );
                    self.num_holes -= 1;
                    self.init();
                } else {
                    tracing::info!(
                        "Tried to decrease number of moles but already at {} minimum!",
                        MIN_MOLES
                    )
                }
            }
            KeyCode::Char(']') => {
                if self.num_moles < MAX_MOLES {
                    tracing::info!(
                        "Increasing number of moles from {} to {}",
                        self.num_moles,
                        self.num_moles + 1
                    );
                    self.num_moles += 1;
                    self.moles = init_moles(self.num_moles);
                    tracing::info!(
                        "Increasing number of holes from {} to {}",
                        self.num_holes,
                        self.num_holes + 1
                    );
                    self.num_holes += 1;
                    self.holes = init_holes(self.num_holes);
                } else {
                    tracing::info!(
                        "Tried to increase number of moles but already at {} maximun!",
                        MAX_MOLES
                    )
                }
                self.name_and_assign_moles();
            }
            _ => {}
        }
    }

    // give each mole a name and put it in a hole
    fn name_and_assign_moles(&mut self) {
        tracing::info!(
            "Naming {} moles and assigning them to holes",
            self.num_moles
        );
        for i in 0..self.moles.len() {
            self.moles[i].name = self.get_mole_name();

            // assign the mole to a random unoccupied hole
            let mut rng = rand::rng();
            let chosen_hole = self
                .holes
                .iter()
                .enumerate()
                .filter(|(_, h)| !h.occupied)
                .map(|(i, _)| i)
                .choose(&mut rng);

            if let Some(index) = chosen_hole {
                self.moles[i].hole_index = index;
                self.holes[index].occupied = true;
                tracing::debug!(
                    "Assigned mole {} to hole at x/y {}/{}",
                    self.moles[i].name,
                    self.holes[index].pos_x,
                    self.holes[index].pos_y
                );
            }
        }
    }

    // get a random mole name from the list
    fn get_mole_name(&mut self) -> String {
        let index = rand::random_range(0..self.mole_names.len());
        let name = String::from(&self.mole_names[index]);

        // remove to prevent name re-use, re-up all names from file if we're out
        self.mole_names.remove(index);
        if self.mole_names.len() == 0 {
            tracing::info!("Ran out of mole names, reloading from {}", MOLE_NAME_FILE);
            read_mole_names(MOLE_NAME_FILE);
        };

        name
    }

    fn shutdown(&mut self) {
        self.exit = true;
        let _ = self.shutdown_tx.send(true);
    }
}

// generate moles with default values
fn init_moles(num_moles: u64) -> Vec<Mole> {
    tracing::info!("Generating {} moles", num_moles);
    (0..num_moles).map(|_| Mole::new()).collect()
}

// generate holes
fn init_holes(num_holes: u64) -> Vec<Hole> {
    tracing::info!("Generating {} holes", num_holes);

    // top row width in holes - total number squared & rounded up
    let holes_wide = (num_holes as f64).sqrt().ceil() as i64;
    let mut holes = vec![];

    let mut x_count = 0;
    let mut y_count = 0;
    let mut odd_row = true;

    // create new holes with x/y logical position on the game board
    for n in 0..num_holes {
        holes.push(Hole::new(x_count, y_count));
        tracing::debug!(
            "Assigned hole #{} to x/y position {}/{}",
            n,
            x_count,
            y_count
        );
        if (odd_row && x_count == holes_wide - 1) {
            x_count = 0;
            y_count += 1;
            odd_row = false;
        } else if (!odd_row && x_count == holes_wide - 2) {
            x_count = 0;
            y_count += 1;
            odd_row = true;
        } else {
            x_count += 1;
        }
    }
    holes
}

fn read_mole_names(filename: &str) -> Vec<String> {
    tracing::info!("Reading mole names from file {}", filename);
    let file = File::open(filename).expect("Couldn't read mole name file {filename}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Couldn't parse line"))
        .collect()
}
