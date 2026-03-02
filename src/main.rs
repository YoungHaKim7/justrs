use std::env;
use std::fs;

const JUSTFILE_TEMPLATE: &str = include_str!("../justfile_template.just");

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let justfile_path = current_dir.join("justfile");

    match fs::write(&justfile_path, JUSTFILE_TEMPLATE) {
        Ok(_) => {
            println!("justfile created successfully at: {}", justfile_path.display());
        }
        Err(error) => {
            eprintln!("Error creating justfile: {}", error);
        }
    }
}
