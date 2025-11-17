use std::process::{Command, Output};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

pub fn execute_shell_command<T>(command: &str) -> Result<T> 
where
    T: std::convert::From<std::string::String>
{
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(format!("Success : {}", stdout).into())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Command failed: {}", stderr).into())
    }
}
