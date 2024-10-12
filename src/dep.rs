//08/10/2024 by Matteo Fava
//Dependency struct

//Imports
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Dependecy {
    pub name: PathBuf,
    pub found_source: bool,
}

impl Dependecy {
    pub fn new(name: &str, found_source: bool) -> Self {
        Self {
            name: PathBuf::from(name),
            found_source,
        }
    }

    pub fn from(name: &Path) -> Self {
        Self {
            name: PathBuf::from(name),
            found_source: false,
        }
    }
}
