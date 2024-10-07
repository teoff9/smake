//03/10/24 by Matteo Fava
//Functions to check

//Imports
use crate::args::{Args, Compiler};
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
