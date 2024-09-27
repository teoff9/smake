// 26/09/2024 by Matteo Fava (teoff9)
// Functions to pars the cpp file

//imports
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

//functions
pub fn parse_file(file_name: &String) -> Vec<String> {
    // Open the file and create a buffered reader
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // Regex to match lines that start with #include "lib.h"
    let include_regex = Regex::new(r#"^\s*#include\s+"([^"]+\.h)"\s*$"#).unwrap();

    // Store the dependencies (lib names without .h)
    let mut includes: Vec<String> = Vec::new();

    // Read lines one by one
    for line in reader.lines() {
        let line = line.unwrap();

        // Check for lines that are exactly #include "lib.h"
        if let Some(captures) = include_regex.captures(&line) {
            // Get the library name without the .h extension
            if let Some(lib) = captures.get(1) {
                let lib_name = lib.as_str().trim_end_matches(".h").to_string();
                includes.push(lib_name);
            }
        }
    }

    includes
}

pub fn check_for_dep_existance(
    rel_path: &str,
    dependencies: &[String],
    fails_i: &mut Vec<usize>,
) -> bool {
    let mut flag = true;
    for (i,dp) in dependencies.iter().enumerate() {
        if !Path::new(&format!("{}/{}.h", rel_path, dp)).exists()
            && !Path::new(&format!("{}/{}.cpp", rel_path, dp)).exists()
        {
            fails_i.push(i);
            flag = false;
        }
    }
    flag
}
