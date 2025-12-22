#[derive(Debug)]
pub enum Error {
    InvalidRegex(String),
}

pub type TodoResult<T> = Result<T, Error>;
