// 26/09/2024 by Matteo Fava (teoff9)
//Simple Make : https://github.com/teoff9/smake.git
//Simple Make : generates a Makefile from a single cpp file.

//Mods
pub mod parser;
pub mod writer;

//Imports
use clap::Parser;
use std::env::current_dir;

//define the arguments to be passed
#[derive(Parser)]
#[command(name = "smake")]
#[command(about = "Simple Makefile: generate a makefile from a .cpp file.")]
struct Args {
    //Path to .cpp file 
    file_name: String
}

fn main() {
    //parse the arguments and get the current directory
    let args = Args::parse();
    let curr_dir = current_dir();
    
    //check the validity of Args.file_name

    //get the dependencies from cpp file

    //write the makefile

    //tell the output 

}
