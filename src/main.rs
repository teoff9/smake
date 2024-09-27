// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Mods
//pub mod logging;
pub mod parser;
pub mod writer;

//Imports
use parser::parse_file;
use std::env::args;
use std::path::Path;
use writer::generate_make_file;

fn main() {
    //collect the args
    // arg 0 is the executable
    // arg 1 is the path given by smake shell command
    // arg n with n>1 are the files
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Smake error: no arguments given. Try: smake file_name.cpp")
        //#TODO: log the output
    } else {
        for i in 2..args.len() {
            //check for the file
            if !Path::new(&format!("{}/{}", &args[1], &args[i])).exists() {
                println!(
                    "Smake error: can't find {}. Try with the complete path.",
                    args[i]
                );
                //#TODO: log the output
                continue;
            }

            //parse the file to get the dependecies
            let dependecies: Vec<String> = parse_file(&format!("{}/{}",args[1],args[i]));

            //generate the Makefile
            generate_make_file(&args[1], &args[i], &dependecies);

            //log the output
            //#TODO: output the logging
        }
    }
}
