//03/10/24 by Matteo Fava
//Errors

//Imports
use thiserror::Error;

//define the errors
#[derive(Debug, Error)]
pub enum SmakeError {
    #[error("Invalid <FILE_NAME>: {0:?} is not a valid path.")]
    InvalidPath(String),

    #[error("Invalid <FILE_NAME>: {0:?} is not a .cpp file.")]
    InvalidFile(String),

    #[error("smake error: {0:?}")]
    InternalError(String),
}
