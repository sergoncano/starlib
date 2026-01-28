use crate::model::coords::Coords;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PlayerMovedTo(Coords),
    PlayerInteracted,
    ExitLevel,
    ShowTip(String),
}
