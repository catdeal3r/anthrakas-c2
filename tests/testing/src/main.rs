use std::io::{self, Write, Read, BufReader, BufRead};
use std::process::{Command, Stdio};

fn main() {
    // Spawn the child process (interactive shell)
    let mut child = Command::new("/bin/sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // capture stderr too
        .spawn()
        .expect("Failed to start the shell");

    // Get the child process's stdin
    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    
    // Spawn a thread to read the child's output
    std::thread::spawn(move || {
        let mut child_stdout = BufReader::new(child.stdout.take().expect("Failed to open stdout"));
        let mut val = String::new();
        
        loop {
            match child_stdout.read_line(&mut val) {
                Ok(0) => break, // End of output
                Ok(_) => {
                    print!("{}", val);
                    val.clear();
                }
                Err(e) => eprintln!("Error reading from child stdout: {}", e),
            }
        }
    });

    // Main loop to read user input and send it to the shell
    loop {
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF or empty input (Ctrl-D)
            Ok(_) => {
                if let Err(e) = child_stdin.write_all(input.as_bytes()) {
                    eprintln!("Error writing to child stdin: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
}
