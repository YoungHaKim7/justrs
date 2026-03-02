use std::{env, fs, process::Command};
use tempfile::TempDir;

#[test]
fn test_justfile_creation() {
    // Test that justrs creates a justfile in the current directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Change to temp directory
    let original_dir = env::current_dir().expect("Failed to get current dir");
    env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");

    // Run the justrs binary
    let output = Command::new("cargo")
        .args(["run", "--bin", "justrs", "--manifest-path", &format!("{}/Cargo.toml", original_dir.display())])
        .output()
        .expect("Failed to run justrs");

    // Restore original directory
    env::set_current_dir(&original_dir).expect("Failed to restore original dir");

    // Verify the program executed successfully
    assert!(output.status.success(), "Program should execute successfully. stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Verify justfile was created
    let justfile_path = temp_dir.path().join("justfile");
    assert!(justfile_path.exists(), "justfile should be created");

    // Verify justfile contains expected content
    let content = fs::read_to_string(&justfile_path).expect("Failed to read justfile");
    assert!(content.contains("project_name :="), "justfile should contain project_name variable");
    assert!(content.contains("# cargo run"), "justfile should contain run recipe");
}

#[test]
fn test_justfile_template_validity() {
    // Test that the generated justfile has valid syntax
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Change to temp directory
    let original_dir = env::current_dir().expect("Failed to get current dir");
    env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");

    // Run justrs
    let _ = Command::new("cargo")
        .args(["run", "--bin", "justrs", "--manifest-path", &format!("{}/Cargo.toml", original_dir.display())])
        .output()
        .expect("Failed to run justrs");

    // Restore original directory
    env::set_current_dir(&original_dir).expect("Failed to restore original dir");

    // Try to list recipes using 'just' (if available)
    let just_output = Command::new("just")
        .args(["-l", "--unstable"])
        .current_dir(temp_dir.path())
        .output();

    // If 'just' is installed, verify the justfile is valid
    if let Ok(output) = just_output {
        assert!(output.status.success(), "Generated justfile should be valid. stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
