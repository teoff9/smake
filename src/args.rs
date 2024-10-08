//03/10/24 by Matteo Fava
//Arguments of smake

//Imports
use crate::errors::SmakeError;
use clap::{Parser, ValueEnum};

//define the arguments to be passed
#[derive(Parser)]
#[command(name = "smake")]
#[command(about = "Simple Makefile: generate a makefile from a .cpp or .c file.")]
pub struct Args {
    //Path to .cpp file
    #[arg(help = "Relative or absolute path to a .cpp or .c file.")]
    pub target: String,

    //Additional compiler arguments
    #[arg(
        long,
        num_args=0..,
        value_delimiter=',',
        help = "Additional compiler arguments used every time the choosen compiler is called in the makefile.\nFor multiple arguments: \"arg1, arg2...\""
    )]
    pub args: Vec<String>,

    //Compiler choice between gcc and g++
    #[arg(long, short, default_value="g++",value_parser=parse_compiler, help="Choose between gcc and g++ compilers.")]
    pub compiler: Compiler,

    //Verbose: prints what is doing..
    #[arg(
        long,
        short,
        default_value = "false",
        help = "Verbose mode: prints what is doing."
    )]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Compiler {
    Gcc,
    Gpp,
}

pub fn parse_compiler(c: &str) -> Result<Compiler, SmakeError> {
    match c {
        "gcc" => Ok(Compiler::Gcc),
        "g++" => Ok(Compiler::Gpp),
        "gpp" => Ok(Compiler::Gpp),
        _ => Err(SmakeError::InvalidCompiler(c.to_owned())),
    }
}
