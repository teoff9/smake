//03/10/24 by Matteo Fava
//Arguments of smake

//Imports
use clap::Parser;

//define the arguments to be passed
#[derive(Parser)]
#[command(name = "smake")]
#[command(about = "Simple Makefile: generate a makefile from a .cpp file.")]
pub struct Args {
    //Path to .cpp file
    #[arg(help = "Relative or absolute path to a .cpp file.")]
    pub file_name: String,
}
