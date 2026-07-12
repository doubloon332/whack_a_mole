// Hole position & occupancy - used by Game
// hole origin is 0,0 at top left

#[derive(Debug)]
pub struct Hole {
    pub pos_x: i64,
    pub pos_y: i64,
    pub occupied: bool,
}

impl Hole {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            pos_x: x,
            pos_y: y,
            occupied: false,
        }
    }
}
