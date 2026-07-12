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

// Indexed 256 color for Ratatui (for cross-term compatability; not all support RGB)
const BROWN: u8 = 58;

#[derive(Debug)]
pub struct GameDisplay {
    view_min_x: f64,
    view_max_x: f64,
    view_min_y: f64,
    view_max_y: f64,
}

impl GameDisplay {
    pub fn new() -> Self {
        GameDisplay {
            view_min_x: VIEWPORT_MIN_X,
            view_max_x: VIEWPORT_MAX_X,
            view_min_y: VIEWPORT_MIN_Y,
            view_max_y: VIEWPORT_MAX_Y,
        }
    }

    // given a render frame and an area within it, draw the contents of the game snapshot
    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect, snapshot: &BoardSnapshot) {
        // test paint
        let game_canvas = Canvas::default()
            .x_bounds([self.view_min_x, self.view_max_x])
            .y_bounds([self.view_min_y, self.view_max_y])
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
