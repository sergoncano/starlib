//! Contains the generic trait [Entity].
use std::time::Duration;

use crate::{
    Sprite,
    model::{coords::Coords, event::Event, stage::Stage},
};

/// Trait that describes the behavior of a struct in the game loop. [Level](crate::Level)'s [run](crate::Level::run) method takes care of
/// calling the methods implemented here. The generic parameter is the type of the user event the
/// entity can process. You're most likely going to envelope any struct that implements this in a
/// [Box], so you can pass it to a [Level](crate::Level).
pub trait Entity<T> {
    /// The entity takes a turn. The return value is a vector of events (possibly none) the entity
    /// sends when it has finished the turn.
    fn take_turn(&mut self, stage: &mut Stage) -> Vec<Event<T>>;

    /// The entity reacts to a user event, possibly sending back more events in the process.
    /// It does not process system events since those are reserved for the level, but it may return
    /// them.
    fn handle_event(&mut self, event: &T, stage: &mut Stage) -> Vec<Event<T>>;

    /// Returns the delay until next turn in a [Duration](std::time::Duration) format. This may dynamically update, hence an Entity might speed
    /// up or slow down its turns during the game.
    fn get_turn_delay(&self) -> Duration;

    /// Returns a pair of the entity's coords and their sprite.
    fn get_render(&self) -> (Coords, Sprite);
}
