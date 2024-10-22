// 26/09/2024 by Matteo Fava (teoff9)
// Functions to parse files

//Imports
use crate::{dep::Dependecy, errors::SmakeError};
use regex::Regex;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use super::checks::resolve_deps;

//Parse a .cpp or .c file
pub fn parse_cpp_file(target: &Path) -> anyhow::Result<Vec<Dependecy>> {
    //open the file and load it to a string removing the commented blocks
    let f = remove_commented_blocks(&read_to_string(target).map_err(|_| {
        SmakeError::CantOpenFile(
            target
                .to_str()
                .expect("Should be able to convert path to string!")
                .to_string(),
        )
    })?);
    //search for the dependencies
    let mut deps: Vec<Dependecy> = vec![];
    let lib_regex = Regex::new(r#""([^"]+\.h)""#).expect("Can't unwrap regex");
    for line in f.lines().map(|l| l.trim().replace(" ", "")) {
        if line.starts_with("#include") {
            for lib in line.replace("#include", "").split(",") {
                if let Some(e) = get_lib(lib, &lib_regex) {
                    deps.push(Dependecy::from(&e, None));
                }
            }
        }
    }
    Ok(deps)
}

//remove the commented blocks of a file
pub fn remove_commented_blocks(f: &str) -> String {
    let blocks_regex = Regex::new(r"/\*[\s\S]*?\*/").expect("Can't unwrap regex");
    blocks_regex.replace_all(f, "").to_string().to_owned()
}

//get libs from a line
pub fn get_lib(lib: &str, regex: &Regex) -> Option<PathBuf> {
    if let Some(captures) = regex.captures(lib) {
        if let Some(l) = captures.get(1) {
            return Some(PathBuf::from(l.as_str()));
        }
    }
    None
}

//search the source files of deps in the same folder as the header
//if found, parse it and add it's dependencies to the list
pub fn search_and_parse_dependecies(deps: &mut Vec<Dependecy>,dir: &Path,verbose: bool,) -> anyhow::Result<()> {
    let mut tmp: Vec<Dependecy> = vec![];
    let mut dir = dir.to_path_buf();
    for d in deps {
        let mut exp: Vec<Dependecy> = vec![];
        explore_deps(d, &mut dir, &mut exp, verbose)?;
        resolve_deps(&mut exp, &dir, verbose)?;
        tmp.append(&mut exp);
    }
    //check if is already in deps, then add the dependencies found (check also tmp)

    Ok(())
}

//Recursive function: explore the dependency of a file
pub fn explore_deps(d: &Dependecy,dir: &mut Path,exp: &mut Vec<Dependecy>,verbose: bool,) -> anyhow::Result<()> {
    if verbose {
        println!("Searching in: {}", d.name.display())
    }
    //trova dependency in d
    let mut tmp = parse_cpp_file(&dir.join(&d.name))?;
    //tieni quelle che esistono
    resolve_deps(&mut tmp, dir, verbose)?;

    for new_d in &mut tmp {
        let mut new_dir = dir
            .join(new_d.name.parent().expect("Can't get parent!"))
            .canonicalize()?;
        explore_deps(&new_d, &mut new_dir, exp, verbose)?;
    }

    for new_d in &tmp {
        todo!()
    }
    exp.append(&mut tmp);

    Ok(())
}
