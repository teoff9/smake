// 26/09/2024 by Matteo Fava (teoff9)
// Functions to parse files

//Imports
use crate::errors::SmakeError;
use std::path::{Path, PathBuf};

//Functions

//Parse a .cpp file
pub fn parse_cpp_file(target: &Path) -> Result<Vec<PathBuf>, SmakeError> {
    todo!()
}

//Parse a makefile
pub fn parse_makefile() -> Result<String, SmakeError> {
    todo!()
}
