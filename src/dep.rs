//08/10/2024 by Matteo Fava
//Dependency struct

//Imports
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Dependecy {
    pub name: PathBuf,
    pub dependencies: Option<Vec<PathBuf>>,
    pub found_source: bool,
}

impl Dependecy {
    pub fn new(name: &str, dependencies: Option<Vec<PathBuf>>, found_source: bool) -> Self {
        Self {
            name: PathBuf::from(name),
            dependencies,
            found_source,
        }
    }

    pub fn from(name: &Path) -> Self {
        Self {
            name: PathBuf::from(name),
            dependencies: None,
            found_source: false,
        }
    }

    pub fn add_dependency(&mut self, dep: &Path) {
        if let Some(l) = &mut self.dependencies {
            l.push(PathBuf::from(dep));
        } else {
            self.dependencies = Some(vec![dep.to_path_buf()]);
        }
    }
}
