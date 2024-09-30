// 26/09/2024 by Matteo Fava (teoff9)
// Functions to write the Makefile

//imports
use std::{fs::OpenOptions, io::Write};

pub fn generate_make_file(path: &str, file_name: &str, dependencies: &Vec<String>) {
    // Open the makefile in append mode
    let make_name = format!("{}/{}", path, "makefile");

    let mut f = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(&make_name)
    {
        Ok(f) => f,
        Err(_) => {
            println!("Smake error: failed to open or create the makefile in {path}");
            return;
        }
    };

    // Get the file name and dependencies
    let file_name = file_name.strip_suffix(".cpp").unwrap();
    let deps = generate_dependencies(dependencies);
    let comp_deps = generate_compile_dependencies(dependencies);

    // Prepare the makefile content
    let content = format!(
        "{file_name}: {file_name}.o {deps}
\tg++ {file_name}.o {deps} -o {file_name}\n
{file_name}.o: {file_name}.cpp\n\
\tg++ -c {file_name}.cpp -o {file_name}.o\n\
{comp_deps}\nclean:\n\trm {deps}\n\trm *.o {file_name}\n
"
    );

    // Write the content to the file, appending to it
    f.write_all(content.as_bytes())
        .unwrap_or_else(|_| panic!("Smake error: failed to write to file {make_name}"));
}

fn generate_dependencies(deps: &[String]) -> String {
    let mut d = String::new();
    for dp_name in deps {
        d.push_str(&format!("{dp_name}.o "));
    }
    d
}

fn generate_compile_dependencies(deps: &Vec<String>) -> String {
    let mut comp_deps = String::new();
    for dp_name in deps {
        let comp = format!("{dp_name}.o: {dp_name}.cpp\n\tg++ -c {dp_name}.cpp -o {dp_name}.o\n");
        comp_deps.push_str(&comp);
    }
    comp_deps
}
