#[derive(Debug, Clone)]
pub enum Loadable<T> {
    Idle,
    Loading,
    Loaded(T),
    Error(String),
}
