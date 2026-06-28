// Message formats for cross-task communication

#[derive(Debug)]
pub enum RenderMessage {
    Panel(PanelUpdate),
    Shutdown,
}

#[derive(Debug)]
pub struct Panel {
    pub panel_kind: PanelKind,
    pub title: String,
    pub text: String,
    pub border_color: String, // from ratatui::Color
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
