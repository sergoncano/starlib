pub mod player;
pub mod rabbit;

use crate::{event::EventDriven, renderable::Renderable};

pub trait Entity: Renderable + EventDriven {}
