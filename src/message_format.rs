// Message formats for cross-task communication

use crate::mole::Mole;

#[derive(Debug)]
// message to be passed between Game & Renderer
pub enum RenderMessage {
    Panel(PanelUpdate),
}

// what panel to update & with what - used with RenderMessage
#[derive(Debug)]
pub struct PanelUpdate {
    pub target: PanelKind,
    pub op: PanelOps,
}

#[derive(Debug)]
// differing contents for Debug/Status (text) & Game (game board)
pub enum PanelContent {
    Text(String),
    Board(BoardSnapshot),
}

#[derive(Debug, Clone)]
// game board updates from Game -> Renderer
pub struct BoardSnapshot {
    pub moles: Vec<Mole>,
    pub num_moles: u32,
}

#[derive(Debug)]
pub enum PanelOps {
    Game(GamePanelOp),
    Text(TextPanelOp),
}

// operations to be performed on Status or Debug panel contents
#[derive(Debug)]
pub enum TextPanelOp {
    Append(String),
    Replace(String),
    Clear,
}

#[derive(Debug)]
// operations to be performed on the game board
pub enum GamePanelOp {
    Replace(BoardSnapshot),
    Clear,
}

// Shared understanding of panels for message routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelKind {
    Game,
    Status,
    Debug,
}
