//03/10/24 by Matteo Fava
//Functions to check

//Imports
use crate::errors::SmakeError;
use std::path::{Path, PathBuf};

//Function to check if the path is valid, if it has the prefix required
pub fn check_file(path_to_file: &str, suffix: &str) -> Result<PathBuf, SmakeError> {
    let p = Path::new(path_to_file);
    if !p.exists() {
        Err(SmakeError::InvalidPath(path_to_file.to_owned()))
    } else if !p.ends_with(suffix) {
        Err(SmakeError::InvalidFile(path_to_file.to_owned()))
    } else {
        Ok(p.to_path_buf())
    }
}
