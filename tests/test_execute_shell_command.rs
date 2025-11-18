use justrs::execute_shell_command::execute_shell_command;

#[test]
fn test_execute_shell_command_success() {
    let result = execute_shell_command::<String>("echo 'Success : Hello World'");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success : Hello World");
}

#[test]
fn test_execute_shell_command_failure() {
    let result = execute_shell_command::<String>("exit 1");
    assert!(result.is_err());
}

// #[test]
// fn test_execute_shell_command_with_stderr() {
//     let result = execute_shell_command::<String>(">&2 echo 'error message'; exit 1");
//     assert!(result.is_err());
//     let error_msg = result.unwrap_err();
//     assert!(error_msg.contains("error message"));
// }

#[test]
fn parse_bool_from_command() {
    // prints "true\n" to stdout, parse -> bool true
    let v = execute_shell_command::<bool>("printf true").expect("should parse bool");
    assert!(v);
}

// #[test]
// fn test_execute_shell_command_empty_output() {
//     let result = execute_shell_command::<bool>(true);
//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), false);
// }

#[test]
fn test_execute_shell_command_multiline() {
    let result = execute_shell_command::<String>("echo 'line1'; echo 'line2'");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("line1"));
    assert!(output.contains("line2"));
}
