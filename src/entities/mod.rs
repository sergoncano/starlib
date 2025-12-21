pub mod player;

use crate::{event::EventDriven, renderable::Renderable};

pub trait Entity: Renderable + EventDriven {}
