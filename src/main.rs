use std::process::{Command, Output};

fn execute_shell_command(command: &str) -> Result<String, String> {
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Command failed: {}", stderr))
    }
}

fn main() {
    let input_str = include_str!("../input.txt");

    match execute_shell_command(input_str) {
        Ok(output) => println!("Output  justfile Done !: {}", output),
        Err(error) => eprintln!("Error : {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_shell_command_success() {
        let result = execute_shell_command("echo 'Hello World'");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World\n");
    }

    #[test]
    fn test_execute_shell_command_failure() {
        let result = execute_shell_command("exit 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_shell_command_with_stderr() {
        let result = execute_shell_command(">&2 echo 'error message'; exit 1");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("error message"));
    }

    #[test]
    fn test_execute_shell_command_empty_output() {
        let result = execute_shell_command("true");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_execute_shell_command_multiline() {
        let result = execute_shell_command("echo 'line1'; echo 'line2'");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("line1"));
        assert!(output.contains("line2"));
    }
}
