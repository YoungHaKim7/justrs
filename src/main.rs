use std::process::{Command, Output};

fn main() {
    let input_str = include_str!("../input.txt");
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(input_str)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output  justfile Done !: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error : {}", stderr)
    }
}
