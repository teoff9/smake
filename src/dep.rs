//08/10/2024 by Matteo Fava
//Dependency struct

//Imports
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Dependecy {
    pub name: PathBuf,
    pub source: Option<PathBuf>,
    pub absolute_path: Option<PathBuf>,
}

impl Dependecy {
    pub fn new(name: &str, source: Option<PathBuf>, absolute_path: Option<&Path>) -> Self {
        Self {
            name: PathBuf::from(name),
            source,
            absolute_path: { absolute_path.map(|e| e.to_path_buf()) },
        }
    }

    pub fn from(name: &Path, absolute_path: Option<&Path>) -> Self {
        Self {
            name: PathBuf::from(name),
            source: None,
            absolute_path: { absolute_path.map(|f| f.to_path_buf()) },
        }
    }

    pub fn search_source(&mut self, dir: &Path) -> bool {
        if dir.join(self.name.with_extension("cpp")).exists() {
            self.source = Some(self.name.with_extension("cpp"));
        } else if dir.join(self.name.with_extension("c")).exists() {
            self.source = Some(self.name.with_extension("c"));
        } else {
            return false;
        }
        true
    }

    pub fn set_abs_path(&mut self, abs_path: PathBuf) -> anyhow::Result<()> {
        self.absolute_path = Some(abs_path.canonicalize()?);
        Ok(())
    }
}
