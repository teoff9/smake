// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Imports
use clap::Parser;
use smake::args::Args;
use smake::file_io::file_checks::check_target;
use smake::file_io::parser::parse_cpp_file;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    //parse the arguments
    let args = Args::parse();
    dbg!(&args.args);
    //check the validity of Args.file_name
    let target: PathBuf = check_target(&args)?.canonicalize()?;

    //parse the target to get the dependencies of target
    let deps_h = parse_cpp_file(&target)?;
    println!(" => Parsed {:?}: found {} dependencies...", args.target, deps_h.len());

    //search for the .h files (if not found remove from deps alerting the user)
    
    
    //if found, parse the .cpp files of the deps to get their depencencies, add them to deps if not already there
    //NOTE: this assumes the .cpp source files are in the same directory as the headers!!

    //write the makefile

    //tell the output

    Ok(())
}
