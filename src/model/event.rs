#[derive(Debug)]
pub enum Event<T> {
    User(T),
    ExitLevel(i32),
}
