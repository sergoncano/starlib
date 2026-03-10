//! Contains the generic enum [Event] used for message passing.
use crate::model::tip::Tip;

/// A system event (that may contain user events), used by entities to communicate among themselves or with the level.
/// The generic parameter is the type of the user event that entities can process.
#[derive(Debug)]
pub enum Event<T> {
    /// A user defined event. The level will not process it, and instead it will send it back to *all*
    /// entities. Including the sender.
    User(T),
    /// A [Tip]. The renderer will show it according to its duration and priority.
    Tip(Tip),
    /// The level changes its stage to the one with index indicated by the usize. There is no
    /// out-of-bounds checking, so the programmer is responsible for doing it themselves. An
    /// out-of-bounds access to the level's stages vector will result in a panic.
    /// Also note that entities aren't affected by this event and if they are to change positions
    /// when the stage is changed, a user event to handle that should be created.
    ChangeStage(usize),
    /// Stops the [run](crate::Level::run) method of the level, resulting in an exit code indicated by
    /// the i32.
    ExitLevel(i32),
}
