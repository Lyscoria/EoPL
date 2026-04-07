#[derive(Debug)]
pub enum RuntimeError {
    TypeError(String),
    NoBindingFound(String),
    EmptyListError(String),
    CondError(String),
    UnpackError(String),
    DivisonByZero(String),
    ArgNumberError(String),
}