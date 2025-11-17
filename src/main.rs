mod execute_shell_command;

use crate::execute_shell_command::execute_shell_command;

fn main() {
    let input_str = include_str!("../input.txt");

    match execute_shell_command::<String>(input_str) {
        Ok(output) => println!("Output  justfile Done !: {}", output),
        Err(error) => eprintln!("Error : {}", error),
    }
}
