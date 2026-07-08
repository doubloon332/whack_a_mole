// GameDisplay

#[derive(Debug)]
pub struct GameDisplay {}

impl Default for GameDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl GameDisplay {
    pub fn new() -> Self {
        Default::default()
    }
}
