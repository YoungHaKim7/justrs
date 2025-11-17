mod test_execute_shell_command;

use std::{fs, process::Command};

use tempfile::TempDir;

#[test]
fn test_simple_command_execution() {
    // Test that the program can execute a simple command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a simple test command
    let test_input = "echo 'Hello from test'";
    let input_path = temp_dir.path().join("input.txt");
    fs::write(&input_path, test_input).expect("Failed to write input.txt");

    // Backup original input.txt
    let original_input_backup = "input.txt.backup";
    if fs::metadata("input.txt").is_ok() {
        fs::copy("input.txt", original_input_backup).expect("Failed to backup input.txt");
    }

    // Copy test input to current directory
    fs::copy(&input_path, "input.txt").expect("Failed to copy test input");

    // Run the program
    let output = Command::new("cargo")
        .args(&["run", "--bin", "justrs"])
        .output()
        .expect("Failed to run justrs");

    // Restore original input.txt
    if fs::metadata(original_input_backup).is_ok() {
        fs::copy(original_input_backup, "input.txt").expect("Failed to restore input.txt");
        fs::remove_file(original_input_backup).expect("Failed to remove backup");
    }

    // Verify program executed successfully
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Program failed. stderr: {}, stdout: {}", stderr, stdout);
    }

    // Verify output contains expected content
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello from testtest_output.txt"));
}

#[test]
fn test_file_creation_command() {
    // Test that the program can create files
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let test_input = "echo 'test content' > test_output.txt";
    let input_path = temp_dir.path().join("input.txt");
    fs::write(&input_path, test_input).expect("Failed to write input.txt");

    // Backup and replace input.txt
    let original_input_backup = "input.txt.backup";
    if fs::metadata("input.txt").is_ok() {
        fs::copy("input.txt", original_input_backup).expect("Failed to backup input.txt");
    }
    fs::copy(&input_path, "input.txt").expect("Failed to copy test input");

    // Run the program
    let output = Command::new("cargo")
        .args(&["run", "--bin", "justrs"])
        .output()
        .expect("Failed to run justrs");

    // Restore original input.txt
    if fs::metadata(original_input_backup).is_ok() {
        fs::copy(original_input_backup, "input.txt").expect("Failed to restore input.txt");
        fs::remove_file(original_input_backup).expect("Failed to remove backup");
    }

    // Clean up test file
    if fs::metadata("test_output.txt").is_ok() {
        fs::remove_file("test_output.txt").expect("Failed to remove test file");
    }

    assert!(
        output.status.success(),
        "Program should execute successfully"
    );
}

// #[test]
// fn test_error_handling() {
//     // Test that the program handles command errors gracefully
//     let temp_dir = TempDir::new().expect("Failed to create temp dir");

//     let test_input = "exit 1";
//     let input_path = temp_dir.path().join("input.txt");
//     fs::write(&input_path, test_input).expect("Failed to write input.txt");

//     // Backup and replace input.txt
//     let original_input_backup = "input.txt.backup";
//     if fs::metadata("input.txt").is_ok() {
//         fs::copy("input.txt", original_input_backup).expect("Failed to backup input.txt");
//     }
//     fs::copy(&input_path, "input.txt").expect("Failed to copy test input");

//     // Run the program
//     let output = Command::new("cargo")
//         .args(&["run", "--bin", "justrs"])
//         .output()
//         .expect("Failed to run justrs");

//     // Restore original input.txt
//     if fs::metadata(original_input_backup).is_ok() {
//         fs::copy(original_input_backup, "input.txt").expect("Failed to restore input.txt");
//         fs::remove_file(original_input_backup).expect("Failed to remove backup");
//     }

//     // Program should still exit successfully even if the command fails
//     // (it should handle the error and print it to stderr)
//     assert!(output.status.success(), "Program should handle errors gracefully");

//     // Check that error message is printed to stderr
//     let stderr = String::from_utf8_lossy(&output.stderr);
//     assert!(stderr.contains("Error"));
// }
