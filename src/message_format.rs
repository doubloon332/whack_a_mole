// Message formats for cross-task communication

use crate::renderer;

#[derive(Debug)]
// Format for telling the Renderer to update a panel's text
pub enum DisplayTextUpdate {
    Text(String),
    Exit,
}
