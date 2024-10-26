// 26/09/2024 by Matteo Fava (teoff9)
// Functions to write the Makefile

//Imports
use crate::{args::Compiler, dep::Dependecy};
use std::{fs::OpenOptions, io::Write, path::Path};

//Write the makefile
pub fn write_makefile(
    target: &Path,
    dir: &Path,
    deps: &Vec<Dependecy>,
    compiler: Compiler,
    args: &[String],
) -> anyhow::Result<()> {
    //open the file
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(dir.join("makefile"))?;

    //get the compiler
    let cmp: &str = match compiler {
        Compiler::Gcc => "gcc",
        _ => "g++",
    };

    //append the variables commands and generate the args str
    let args_string: String = args
        .iter()
        .enumerate()
        .map(|(i, c)| {
            f.write_fmt(format_args!("ARG{} := $({})\n", i, c))?;
            Ok::<String, std::io::Error>(format!("$(ARG{})", i))
        })
        .collect::<Result<Vec<String>, _>>()?
        .join(" ");

    //target name
    let tgt_name = target
        .file_stem()
        .expect("Can't get file name.")
        .to_str()
        .expect("Can't get string");
    //sources list
    let sources_list = deps
        .iter()
        .map(|d| {
            if let Some(s) = &d.source {
                s.with_extension("o")
                    .to_str()
                    .expect("Can't get string from source path")
                    .to_string()
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    //all deps list
    let deps_list = deps
        .iter()
        .map(|d| {
            if let Some(p) = &d.source {
                p.with_extension("o")
                    .to_str()
                    .expect("Can't get string from source path").to_string()
            } else {
                d.name
                    .to_str()
                    .expect("Can't get deps name as str")
                    .to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    //append the main command
    f.write_fmt(format_args!(
        "\n{0}: {0}.o {1}\n\t{cmp} -o {0} {0}.o {2} {3}\n\n{0}.o: {4}\n\t{cmp} -c {4} {3}\n",
        tgt_name,
        deps_list,
        sources_list,
        &args_string,
        target
            .file_name()
            .expect("Can't get file name")
            .to_str()
            .expect("Can't convert file name to str")
    ))?;

    //append the commands
    for d in deps {
        if d.source.is_some() {
            f.write_all(generate_compile_command(d, cmp, &args_string).as_bytes())?;
        }
    }

    //write the clean command
    f.write_fmt(format_args!(
        "\nclean:\n\trm {sources_list}\n\trm {tgt_name}\n"
    ))?;

    Ok(())
}

//Only for the dependecies with a source file
fn generate_compile_command(dep: &Dependecy, compiler: &str, args: &str) -> String {
    format!(
        "\n{0}.o: {1} {2}\n\t{compiler} -c {1} {args}\n",
        dep.name
            .file_stem()
            .expect("Can't get file stem of dependecy")
            .to_str()
            .expect("Can't get deps name as str"),
        dep.name.to_str().expect("Can't get deps name as str"),
        dep.source
            .as_ref()
            .unwrap()
            .to_str()
            .expect("Can't get deps name as str")
    )
}
