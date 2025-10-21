
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

pub fn parse_post_toml(data: &String) -> (Commands, ParseError) {
    match toml::from_str(&data) {
        Ok(out) => (out, ParseError::None),
        Err(_) => {
            let output = format!("({}): POST request's toml is malformed.", colored::Colorize::red("Error"));
            println!("{}", output);
            return (Commands { key: "".to_string(), commands: vec![format!("{}\n", output)] }, ParseError::MalformedToml)
        }
    }
}


pub fn parse_commands_history_file(data: &String) -> (Commands , ParseError) {
    match toml::from_str(&data) {
        Ok(out) => (out, ParseError::None),
        Err(_) => {
            let output = format!("({}): Malformed server-internal commands file.", colored::Colorize::red("Error"));
            println!("{}", output);
            return (Commands { key: "".to_string(), commands: vec![format!("{}\n", output)] }, ParseError::MalformedToml)
        }
    }
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Config {
    pub master_key: String
}


#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Commands {
   pub key: String,
   pub commands: Vec<String>,
}

#[derive(PartialEq)]
pub enum ParseError {
    MalformedToml,
    None
}
