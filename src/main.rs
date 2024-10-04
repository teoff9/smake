// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Mods
pub mod args;
pub mod errors;
pub mod file_checks;
pub mod parser;
pub mod writer;

//Imports
use args::Args;
use clap::Parser;
use file_checks::check_file;
use parser::parse_cpp_file;
use std::{env::current_dir,  path::PathBuf};

fn main() -> anyhow::Result<()> {
        //parse the arguments and get the current directory
        let args = Args::parse();
        let curr_dir: PathBuf = current_dir()?;

        //check the validity of Args.file_name then create the relative path from current dir
        let target: PathBuf = check_file(&args.file_name, ".cpp")?;
    
        //parse the target to get the dependencies
        
        //write the makefile
    
        //tell the output
    
        Ok(())
    }

