// 26/09/2024 by Matteo Fava (teoff9)
// Functions to pars the cpp file

//imports
use regex::Regex;
use std::fs::{File};
use std::io::{BufRead, BufReader};

//functions
pub fn parse_file(file_name: &String) -> Vec<String> {
    //file and reader
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    //regex per #include "lib.h"
    let include_regex = Regex::new(r#"#include\s+"([^"]+)""#).unwrap();

    //dependecies
    let mut includes: Vec<String> = Vec::new();

    //read lines
    for line in reader.lines() {
        let line = line.unwrap();

        // Check for #include "lib.h"
        if let Some(captures) = include_regex.captures(&line) {
            // Get lib name
            if let Some(lib) = captures.get(1) {
                let lib_name = lib.as_str().trim_end_matches(".h").to_string();
                includes.push(lib_name);
            }
        }
    }
    includes
}
