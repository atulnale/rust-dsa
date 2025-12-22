use std::{fs::{File, OpenOptions}, io::Write, path::PathBuf};

mod arrays;
mod binary_search;
mod linked_list;
mod leetcode;

fn main() {
    let inputs: Vec<String> = std::env::args().collect();
    if inputs.len() < 2 {
        panic!("Invalid args");
    }

    // Ensure .rs extension
    let mut file_name = inputs[1].clone();
    if !file_name.ends_with(".rs") {
        file_name.push_str(".rs");
    }

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("leetcode");


    // Create leetcode/<file>.rs
    File::create(path.join(&file_name))
        .expect("Failed to create file");

    // Open mod.rs in append mode
    let mut mod_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.join("mod.rs"))
        .unwrap();

    // Remove `.rs` safely
    let module_name = file_name.strip_suffix(".rs").unwrap();

    // Write on a new line
    writeln!(mod_file, "pub mod {};", module_name).unwrap();

    println!("Created leetcode/{} and updated mod.rs", file_name);
}