// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Imports
use clap::Parser;
use smake::args::Args;
use smake::file_io::checks::{check_target, find_sources, resolve_deps};
use smake::file_io::parser::parse_cpp_file;
use smake::file_io::writer::write_makefile;
use std::{env::current_dir, path::PathBuf};

fn main() -> anyhow::Result<()> {
    //parse the arguments
    let args = Args::parse();

    //check the validity of Args.file_name
    let target: PathBuf = check_target(&args)?;

    //find the absolute path to the directory containing target
    let dir: PathBuf = current_dir()?.join(
        target
            .parent()
            .expect("Can't get parent directory of target."),
    );

    //parse the target to get the dependencies of target
    let mut deps = parse_cpp_file(&target)?;
    if args.verbose {
        println!(
            " => Parsed {:?}: found {} dependencies...",
            args.target,
            deps.len()
        );
    }

    //search for the .h files (if not found remove from deps alerting the user)
    resolve_deps(&mut deps, &dir, args.verbose)?;

    //search sources
    find_sources(&mut deps, &dir, args.verbose)?;

    //write the makefile
    write_makefile(&target, &dir, &deps, args.compiler, &args.args)?;

    println!(" => CREATED makefile in {}", dir.display());

    Ok(())
}
