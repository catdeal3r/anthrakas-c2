
use colored::Colorize;

pub const HELP_STR: &str = r#"Available commands:
- list: Lists connected targets and whether they are online.
- info [target]: Gets system information about [target].
- run [target]: Runs the command/s presented by the user in the next line on [target].
- screenshot [target]: Takes a screenshot of [target] and uploads the result.
- exit: Exits the program.
"#;

pub fn output_line_response(line: String) {
    println!("[{}]\n{}\n", ">".bold().yellow(), line);
}

pub fn output_line_sys(line: String) {
    println!("[{}]\n{}\n", "sys".bold().blue(), line);
}
