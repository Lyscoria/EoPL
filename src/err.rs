#[derive(Debug)]
pub enum RuntimeError {
    TypeError(String),
    NoBindingFound(String),
    EmptyListError(String),
    CondError(String),
    UnpackError(String),
    DivisionByZero(String),
    ArgNumberError(String),
    BeginError,
}