//03/10/24 by Matteo Fava
//Functions to check file existance and stuff

//Imports
use crate::args::{Args, Compiler};
use crate::dep::Dependecy;
use crate::errors::SmakeError;
use std::path::{Path, PathBuf};

//Function to check if the path is valid, if it has the prefix required
pub fn check_target(args: &Args) -> Result<PathBuf, SmakeError> {
    let p = Path::new(&args.target);
    if !p.exists() && !p.is_file() {
        Err(SmakeError::InvalidTarget(args.target.to_owned()))
    } else {
        let e = p
            .extension()
            .ok_or_else(|| SmakeError::InvalidFile(args.target.to_owned()))?;
        if e == "c" {
            Ok(p.to_path_buf())
        } else if e == "cpp" {
            match args.compiler {
                Compiler::Gcc => Err(SmakeError::InvalidChoice {
                    target: args.target.to_owned(),
                    compiler: String::from("gcc"),
                }),
                _ => Ok(p.to_path_buf()),
            }
        } else {
            Err(SmakeError::InvalidFile(args.target.to_owned()))
        }
    }
}

//Function to check if dependencies exists, if one doesn't remove it
pub fn resolve_deps_h(deps_h: &mut Vec<Dependecy>, dir: &Path, verbose: bool) {
    deps_h.retain(|e| {
        if !dir.join(&e.name).exists() {
            if verbose {
                println!(
                    " => Couldn't find: \"{}\". It won't be included in the makefile.",
                    e.name.display()
                );
            }
            false
        } else {
            true
        }
    });
}