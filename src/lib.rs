//! A crate that acts as a 2D grid-based TUI game engine.
//!
//! starlib provides abstractions for creating entities, handling collisions, sprite layering,
//! message passing among elements of the game, getting user input, creating UIs and rendering.
//!
//! A small example of what can be built with this crate can be found in the `examples` folder.
//! Run `cargo run --example main` to see it firsthand.
//!
//! The starting structure for any starlib game should be something like this:
//! ```rust, ignore
//! starlib::renderer::setup_terminal_properties();
//! // Create stages and entities...
//! let mut level = starlib::Level::build(stages, entities);
//! level.run();
//! starlib::renderer::restore_terminal_properties();
//! ```
//! [Stages](Stage) are the environments in which the game happens. [Entities](Entity) are elements that react to each
//! other and the environment. The player is likely one of them (However, there needs to be no
//! player, you can also use starlib to run simulations).
//! Since [Entity] is a trait, you need to implement it for a struct of your choosing. Check out
//! its documentation to see how.

pub mod graphics;
pub mod model;
pub mod util;

pub use graphics::map::Map;
pub use graphics::renderer;
pub use graphics::sprite::Sprite;
pub use graphics::sprite::sprite_vector_from_lines;
pub use model::collider::Collider;
pub use model::collider::collider_vector_from_lines;
pub use model::coords::Coords;
pub use model::entity::Entity;
pub use model::event::Event;
pub use model::level::Level;
pub use model::stage::Stage;
pub use model::tip::Tip;
pub use util::input::Input;
pub use util::input;
pub use util::ui::menu::Menu;
pub use util::ui::title::Title;

pub(crate) const FRAME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000 / 30);
