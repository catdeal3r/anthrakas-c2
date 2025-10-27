use std::collections::HashMap;
use std::fs;
use toml;


pub fn get_files_contents(filepath: &str) -> String {
    if !std::path::Path::exists(std::path::Path::new(&filepath)) {
        println!("({}): `{}` is not found.", colored::Colorize::red("Error"), filepath);
        std::process::exit(1);
    }

    fs::read_to_string(filepath).unwrap()
}

pub fn parse_config(data: &String) -> Config {
    match toml::from_str(&data) {
        Ok(out) => out,
        Err(_) => {
            println!("({}): Malformed config file.", colored::Colorize::red("Error"));
            std::process::exit(1);
        }
    }
}

pub fn parse_post_toml(data: &String) -> (Clients, ParseError) {
    match toml::from_str(&data) {
        Ok(out) => (out, ParseError::None),
        Err(_) => {
            let output = format!("({}): POST request's toml is malformed.", colored::Colorize::red("Error"));
            println!("{}", output);

            let mut clients_map = HashMap::new();

            let client = Client {
                commands: vec![format!("{}\n", output)]
            };
            clients_map.insert("".to_string(), client);

            
            let clients = Clients {
                clients: clients_map
            };
        
            return (clients, ParseError::MalformedToml)
        }
    }
}


pub fn parse_commands_history_file(data: &String) -> (Clients, ParseError) {
    match toml::from_str(&data) {
        Ok(out) => (out, ParseError::None),
        Err(_) => {
            let output = format!("({}): Malformed server-internal commands file.", colored::Colorize::red("Error"));
            println!("{}", output);

            let mut clients_map = HashMap::new();

            let client = Client {
                commands: vec![format!("{}\n", output)]
            };
            clients_map.insert("".to_string(), client);

            
            let clients = Clients {
                clients: clients_map
            };
        
            return (clients, ParseError::MalformedToml)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub keys: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Clients {
    pub clients: HashMap<String, Client>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub commands: Vec<String>
}

#[derive(PartialEq)]
pub enum ParseError {
    MalformedToml,
    None
}
