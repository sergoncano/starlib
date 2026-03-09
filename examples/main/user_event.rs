use starlib::Coords;

pub enum UserEvent {
    PlayerMoved(Coords),
    PlayerInteracted,
    ChangedStage(usize),
}
