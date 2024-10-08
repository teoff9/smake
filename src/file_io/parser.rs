// 26/09/2024 by Matteo Fava (teoff9)
// Functions to parse files

//Imports
use crate::{dep::Dependecy, errors::SmakeError};
use regex::Regex;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

//Parse a .cpp or .c file
pub fn parse_cpp_file(target: &Path) -> Result<Vec<Dependecy>, SmakeError> {
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
            line.replace("#include", "").split(",").for_each(|lib| {
                if let Some(e) = get_lib(lib, &lib_regex) {
                    deps.push(Dependecy::from(&e));
                }
            });
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

//search the source files of deps_h in the same folder as the header
//if found, parse it and add it's dependencies to the list
pub fn search_and_parse_sources(deps_h: &mut Vec<Dependecy>,dir: &Path,verbose: bool) -> Result<(), SmakeError> {
    for d in deps_h {
        let source_deps = parse_cpp_file(&dir.join(&d.name).with_extension("cpp"))?;
        for sd in &source_deps {
            if sd.name != d.name {
                d.add_dependency(&sd.name);
            }
        }
        if verbose && source_deps.len() != 0 {
            println!(
                " => Parsed {}: found {} unique dependecies.",
                d.name.with_extension("cpp").display(),
                &d.dependencies.as_ref().unwrap().len()
            )
        }
    }
    Ok(())
}

//Parse a makefile
pub fn parse_makefile() -> Result<String, SmakeError> {
    todo!()
}
