// 26/09/2024 by Matteo Fava (teoff9)
// Functions to write the Makefile

//imports
use std::{
    fs::File,
    io::Write,
};

//functions
pub fn generate_make_file(path: &String, file_name: &str, dependencies: &Vec<String>) {
    //open file
    let f_name = format!("{}/{}", path, "makefile");
    let mut f =
        File::create(&f_name).unwrap_or_else(|_| panic!("Smake error: failed to create {f_name}"));

    let file_name = file_name.strip_suffix(".cpp").unwrap();
    let deps = generate_dependencies(dependencies);
    let comp_deps = generate_compile_dependencies(dependencies);
    let content = format!(
        "{file_name}: {file_name}.o {deps}
\tg++ {file_name}.o {deps} -o {file_name}.exe\n
{file_name}.o: {file_name}.cpp
\tg++ -c {file_name}.cpp -o {file_name}.o\n
{comp_deps}
clean:\n\trm *.o *.exe"
    );

    f.write_all(content.as_bytes())
        .unwrap_or_else(|_| panic!("Smake error: failed to write to file {f_name}"));
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
