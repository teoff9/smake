// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Mods
//pub mod logging;
pub mod parser;
pub mod writer;

//Imports
use parser::{check_for_dep_existance, parse_file};
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
    } else if args.len() > 3 {
        println!("Smake error: for version v0.0.1, smake will accept just one file! Sorry!");
    } else {
        for i in 2..args.len() {
            let p = Path::new(&args[i]);
            if !p.exists() {
                println!("Smake error: {} is an invalid path", args[i]);
            } else if p.is_file() {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    if ext == "cpp" {
                        let target_name = match p.file_name() {
                            Some(t) => t.to_str().unwrap(),
                            None => {
                                println!("Smake error: failed to get file name.");
                                return;
                            }
                        };
                        let rel_path = match p.parent() {
                            Some(t) => t.to_str().unwrap(),
                            None => {
                                println!("Smake error: failed to get parent folder of file name.");
                                return;
                            }
                        };
                        let path = format!("{}/{}", args[1], rel_path);

                        //parse the file to get the dependecies
                        let dependecies: Vec<String> =
                            parse_file(&format!("{}/{}", path, target_name));

                        //check if the file has dependecies
                        if dependecies.is_empty() {
                            println!("Smake message: No dependecy found in {}, proceeding creating the makefile..", args[i]);
                        } else {
                            let mut fails_i: Vec<usize> = vec![];
                            //check the existance of libs headers and cpp file in destination
                            if !check_for_dep_existance(&path, &dependecies, &mut fails_i) {
                                for j in fails_i {
                                    println!(
                                        "Smake error: couldn't find the dependency: {} .h or .cpp file.",
                                        dependecies[j]
                                    );
                                }
                                return;
                            }

                            //generate the Makefile
                            generate_make_file(&path, target_name, &dependecies);
                            println!("Smake created makefile in {}", path);
                        }
                    } else {
                        println!("Smake error: {} is not a cpp file.", args[i]);
                    }
                }
            } else {
                println!("Smake error: {} doesn't lead to cpp file.", args[i]);
            }
        }
    }
}
