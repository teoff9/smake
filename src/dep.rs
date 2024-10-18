//08/10/2024 by Matteo Fava
//Dependency struct

//Imports
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Dependecy {
    pub name: PathBuf,
    pub found_source: bool,
    pub absolute_path: Option<PathBuf>
}

impl Dependecy {
    pub fn new(name: &str, found_source: bool, absolute_path: Option<&Path>) -> Self {
        Self {
            name: PathBuf::from(name),
            found_source,
            absolute_path: {match absolute_path {
                Some(e) => Some(e.to_path_buf()),
                _ => None
            }}
        }
    }

    pub fn from(name: &Path, absolute_path: Option<&Path>) -> Self {
        Self {
            name: PathBuf::from(name),
            found_source: false,
            absolute_path: {match absolute_path {
                Some(e) => Some(e.to_path_buf()),
                _ => None
            }}
        }
    }

    pub fn search_source(&mut self, dir: &Path) -> bool {
        if dir.join(self.name.with_extension("cpp")).exists() || dir.join(self.name.with_extension("c")).exists() {
            self.found_source = true;
            return true;
        }
        false
    }

    pub fn set_abs_path(&mut self, abs_path: PathBuf) -> anyhow::Result<()> {
        self.absolute_path = Some(abs_path.canonicalize()?);
        Ok(())
    }
}
