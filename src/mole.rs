// Mole - defines the moles that get hit with Whacker

const MOLE_DEFAULT_NAME: &str = "Ed";
const MOLE_DEFAULT_HP: u32 = 100;
const MOLE_DEFAULT_MIN_DWELL: u32 = 500;
const MOLE_DEFAULT_MAX_DWELL: u32 = 1500;
const MOLE_DEFAULT_STARTING_POS_X: u32 = 0;
const MOLE_DEFAULT_STARTING_POS_Y: u32 = 0;

#[derive(Debug, Clone)]
pub struct Mole {
    name: String,        // human-readable name
    hp: u32,             // hit points
    min_dwell_time: u32, // minimum time the mole will stay up, ms
    max_dwell_time: u32, // maximum time the mole will stay up, ms
    pub is_up: bool,
    pub pos_x: u32,
    pub pos_y: u32,
}

impl Default for Mole {
    fn default() -> Self {
        Self {
            name: String::from(MOLE_DEFAULT_NAME),
            hp: MOLE_DEFAULT_HP,
            min_dwell_time: MOLE_DEFAULT_MIN_DWELL,
            max_dwell_time: MOLE_DEFAULT_MAX_DWELL,
            is_up: false,
            pos_x: MOLE_DEFAULT_STARTING_POS_X,
            pos_y: MOLE_DEFAULT_STARTING_POS_Y,
        }
    }
}

impl Mole {
    pub fn new() -> Self {
        Self::default()
    }
}
