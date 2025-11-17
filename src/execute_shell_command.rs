use std::{process::Command, str::FromStr};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Execute `command` via `sh -c` and parse stdout into `T`.
/// `T` must implement `FromStr` and its error must implement `Display`.
pub fn execute_shell_command<T>(command: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        // parse the stdout (trim trailing newline/whitespace)
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let trimmed = stdout.trim();
        let parsed = trimmed.parse::<T>().map_err(|e| {
            format!(
                "Failed to parse stdout (`{}`) into target type: {}",
                trimmed, e
            )
        })?;
        Ok(parsed)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Command failed: {}", stderr).into())
    }
}
