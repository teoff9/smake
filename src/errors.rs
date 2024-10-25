//03/10/24 by Matteo Fava
//Errors

//Imports
use thiserror::Error;

//define the errors
#[derive(Debug, Error)]
pub enum SmakeError {
    #[error("Invalid <TARGET>: {0} is not a valid path to a file.")]
    InvalidTarget(String),

    #[error("Invalid <FILE_NAME>: {0} is not a .cpp or .c file.")]
    InvalidFile(String),

    #[error("Invalid <COMPILER>: {0} not valid, choose between 'gcc' and 'g++'.")]
    InvalidCompiler(String),

    #[error("Invalid <COMPILER>: can't use {compiler} for {target}.")]
    InvalidChoice { target: String, compiler: String },

    #[error("Can't open file {0}")]
    CantOpenFile(String),

    #[error("Can't read lines in file {0}")]
    CantReadLine(String),
}
