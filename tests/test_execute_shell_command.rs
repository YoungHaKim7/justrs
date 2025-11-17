use justrs::execute_shell_command::execute_shell_command;

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
