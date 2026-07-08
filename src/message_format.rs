// Message formats for cross-task communication

#[derive(Debug)]
pub enum RenderMessage {
    Panel(PanelUpdate),
}

#[derive(Debug)]
pub struct PanelUpdate {
    pub target: PanelKind,
    pub op: PanelOp,
}

#[derive(Debug)]
pub enum PanelOp {
    Append(String),
    Replace(String),
    Clear,
}

// Shared understanding of panels for message routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelKind {
    Game,
    Status,
    Debug,
}
