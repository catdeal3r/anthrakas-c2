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

            (create_clients_error_struct(output.clone()), ParseError::MalformedToml)
        }
    }
}


pub fn parse_commands_history_file(data: &String) -> (Clients, ParseError) {
    match toml::from_str(&data) {
        Ok(out) => (out, ParseError::None),
        Err(_) => {
            let output = format!("({}): Malformed server-internal commands file.", colored::Colorize::red("Error"));
            println!("{}", output);

            (create_clients_error_struct(output.clone()), ParseError::MalformedToml)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub clients: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Clients {
    pub clients: HashMap<String, Client>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub commands: Vec<String>,
    pub outputs: Vec<String>
}

#[derive(serde::Serialize)]
pub struct CommandsOnly<'a> {
    pub commands: &'a Vec<String>,
}


#[derive(serde::Serialize)]
pub struct OutputsOnly<'a> {
    pub outputs: &'a Vec<String>,
}

#[derive(PartialEq)]
pub enum ParseError {
    MalformedToml,
    None
}


// Wrapper fns

pub fn create_history_file_if_not_existing(key: String) {
    if !std::path::Path::exists(std::path::Path::new(".anthrakas_server_his.toml")) {
        let content = format!("[clients.{}]\ncommands = []\noutputs = []", key);
        fs::write(".anthrakas_server_his.toml", &content).unwrap();
    }
}

fn create_clients_error_struct(error: String) -> Clients {
    let mut clients_map = HashMap::new();

    let client = Client {
        commands: vec![format!("{}\n", error)],
        outputs: vec!["".to_string()]
    };
    clients_map.insert("".to_string(), client);

    let clients = Clients {
        clients: clients_map
    };

    clients
}
