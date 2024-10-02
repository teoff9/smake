//02.10.2024 by Matteo Fava
//File containing the definition and the methods related to makefile commands

//Imports
use std::path::PathBuf;

//Struct defining a makefile command
pub struct Command {
    pub target: String,
    pub pre: Vec<PathBuf>,
    pub cmd_type: CmdType,
}

//enum for type of command
pub enum CmdType {
    Clean,
    Main,
    Dependency,
}

impl Command {
    //create new instance of Command
    // Note: if you want a clean, put also the file name as target
    // and as pre all the dependecies
    pub fn new(target: String, pre: Vec<PathBuf>, cmd_type: CmdType) -> Self {
        Self {
            target,
            pre,
            cmd_type,
        }
    }

    pub fn to_string(&self) -> String {
        let pre_str = match self.pre.len() {
            0 => String::new(),
            _ => self
                .pre
                .iter()
                .map(|d| d.to_str().expect("Wasn't able to convert PathBuf to &str."))
                .collect::<Vec<_>>()
                .join(" "),
        };

        let cmd_name = match &self.cmd_type {
            CmdType::Clean => "clean",
            _ => &self.target
        };

        let cmds = match &self.cmd_type {
            CmdType::Clean => format!("\n\trm {pre_str}\n\trm {cmd_name}"),
            _ => todo!()
        };
        

        format!("")
    }
}
