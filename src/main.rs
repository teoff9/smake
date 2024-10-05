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

    //check the validity of Args.file_name
    let target: PathBuf = check_target(&args)?.canonicalize()?;

    //parse the target to get the dependencies of target
    let deps = parse_cpp_file(&target)?;
    dbg!(deps);

    //write the makefile

    //tell the output

    Ok(())
}
