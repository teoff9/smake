// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Imports
use std::{env::current_dir,  path::PathBuf};
use clap::Parser;
use smake::args::Args;
use smake::file_io::file_checks::check_file;
use smake::file_io::parser::parse_cpp_file;

fn main() -> anyhow::Result<()> {
        //parse the arguments
        let args = Args::parse();

        //check the validity of Args.file_name then create the relative path from current dir
        let target: PathBuf = check_file(&args.file_name, ".cpp")?;
        let curr_dir: PathBuf = current_dir()?;

        //TODO: choose the compiler, additional arguments, also for c files
    
        //parse the target to get the dependencies
        
        //write the makefile
    
        //tell the output
    
        Ok(())
    }

