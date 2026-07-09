// GameDisplay - Owned by Renderer; takes a BoardSnapshot and returns

use crate::message_format::BoardSnapshot;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Circle};

const VIEWPORT_MIN_X: f64 = -50.0;
const VIEWPORT_MAX_X: f64 = 50.0;
const VIEWPORT_MIN_Y: f64 = -50.0;
const VIEWPORT_MAX_Y: f64 = 50.0;

// rgb color for Ratatui
const BROWN: u8 = 058;

#[derive(Debug)]
pub struct GameDisplay {
    game_snapshot: BoardSnapshot,
}

impl GameDisplay {
    pub fn new(snap: &BoardSnapshot) -> Self {
        GameDisplay {
            game_snapshot: snap.clone(),
        }
    }

    // return a Ratatui canvas object from the current snapshot (provided by Renderer)
    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let game_canvas = Canvas::default()
            .x_bounds([VIEWPORT_MIN_X, VIEWPORT_MAX_X])
            .y_bounds([VIEWPORT_MIN_Y, VIEWPORT_MAX_Y])
            .marker(Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 10.0,
                    y: 20.0,
                    radius: 20.0,
                    color: Color::Indexed(BROWN),
                })
            });
        frame.render_widget(game_canvas, area);
    }
}
