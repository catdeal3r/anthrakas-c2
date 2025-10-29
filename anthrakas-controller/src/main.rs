use std::io::{self, Write};

use colored::Colorize;
pub mod utils;

fn main() {
    println!("Welcome to anthrakas c2.\n");

    let mut output = String::new();

    utils::send_data_and_add_result_to_str(
        &"https://inerrant-carl-nonhazardous.ngrok-free.dev/add_commands".to_string(),
        &"[clients.cat]\ncommands = [\"back to worrkkkkkk!\"]\noutputs = []".to_string(),
        &mut output
    );

    println!("Output from test: {}", output);

    utils::send_data_and_add_result_to_str(
        &"https://inerrant-carl-nonhazardous.ngrok-free.dev/commands".to_string(),
        &"cat".to_string(),
        &mut output
    );
    
    println!("Output from second test: {}", output);

    loop {
        let mut input = String::new();
        
        print!("[{}] $ ", "<".bold().green());

        // flush to show immediately
        io::stdout().flush().unwrap();

        // read input
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        input = input.trim().to_string();
        
        let command: Vec<String> = input
            .split_whitespace()
            .map(String::from)
            .collect();

        process_input(&command);
    }
}

fn process_input(command: &Vec<String>) {
    if command[0] == "help" {
        utils::output_line_sys(utils::HELP_STR.to_string());
        
    } else if command[0] == "info" {
        if 2 > command.len() {
            utils::output_line_sys("Command \"info\" requires second argument.".to_string());
            return
        }
        utils::output_line_response("Uptime: 2h\nHostname: chromet\nDisk space: 14GB/256GB".to_string());
    } else if command[0] == "exit" {
        std::process::exit(0);
    } else {
        utils::output_line_sys(format!("Unknown command: \"{}\"", command[0]));
    }
}

