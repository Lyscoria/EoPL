#[derive(Debug)]
pub enum RuntimeError {
    TypeError(String),
    NoBindingFound(String),
}