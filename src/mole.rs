// Mole - defines the moles that get hit with Whacker

const MOLE_DEFAULT_NAME: &str = "Ed";
const MOLE_DEFAULT_HP: u32 = 100;
const MOLE_DEFAULT_MIN_DWELL: u32 = 500;
const MOLE_DEFAULT_MAX_DWELL: u32 = 1500;

#[derive(Debug)]
pub struct Mole {
    name: String,        // human-readable name
    hp: u32,             // hit points
    min_dwell_time: u32, // minimum time the mole will stay up, ms
    max_dwell_time: u32, // maximum time the mole will stay up, ms
}

impl Default for Mole {
    fn default() -> Self {
        Self {
            name: String::from(MOLE_DEFAULT_NAME),
            hp: MOLE_DEFAULT_HP,
            min_dwell_time: MOLE_DEFAULT_MIN_DWELL,
            max_dwell_time: MOLE_DEFAULT_MAX_DWELL,
        }
    }
}

impl Mole {
    pub fn new() -> Self {
        Self::default()
    }
}
