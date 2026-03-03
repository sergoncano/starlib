use crate::model::tip::Tip;

#[derive(Debug)]
pub enum Event<T> {
    User(T),
    Tip(Tip),
    ExitLevel(i32),
}
