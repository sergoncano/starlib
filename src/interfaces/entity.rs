use crate::interfaces::{event_driven::EventDriven, renderable::Renderable};

pub trait Entity: Renderable + EventDriven {}
