

use toml;
use std::fs;

use crate::utils;

pub async fn add_commands(data: String, keys: Vec<String>) -> String {
    let (clients_and_key, parsing_error) = utils::parse_post_toml(&data);

    if parsing_error == utils::ParseError::MalformedToml {
        if let Some(client) = clients_and_key.clients.get("") {
            return client.commands[0].clone()
        }
    }

    for (key, client) in &clients_and_key.clients {
    
        if !keys.contains(&key) {
            let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
            println!("{}", error);
            return format!("{}\n", error)
        }

        utils::create_history_file_if_not_existing(key.clone());
    
        let (mut history_clients, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

        if history_parsing_error == utils::ParseError::MalformedToml {
            if let Some(h_client) = history_clients.clients.get("") {
                return h_client.commands[0].clone()
            }
        }
        if !history_clients.clients.contains_key(&key.clone()) {
            history_clients.clients.insert(key.clone(), utils::Client { commands: client.commands.clone(), outputs: Vec::new() } );
        } else {
            if let Some(h_client) = history_clients.clients.get_mut(&key.clone()) {
                h_client.commands.extend(client.commands.clone().into_iter());
            }
        }
        
        let toml_str = toml::to_string(&history_clients).unwrap();
        fs::write(".anthrakas_server_his.toml", &toml_str).unwrap();
    }

    let output = format!("({}): Successfully added new commands.", colored::Colorize::blue("Success"));
    println!("{}", output);
    format!("{}\n", output)
}

pub async fn commands(data: String, keys: Vec<String>) -> String {
    
    if !keys.contains(&data) {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    utils::create_history_file_if_not_existing(data.clone());

    let (history_clients, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

    if history_parsing_error == utils::ParseError::MalformedToml {
        if let Some(h_client) = history_clients.clients.get("") {
            return h_client.commands[0].clone()
        }
    }

    if let Some(client) = history_clients.clients.get(&data) {
        println!("({}): Successfully displayed all commands.", colored::Colorize::blue("Success"));

        let wrapper = utils::CommandsOnly {
            commands: &client.commands
        };
        
        return toml::to_string(&wrapper).unwrap()
    } else {
        return "commands = []\n".to_string()
    }
}

pub async fn clear_commands(data: String, keys: Vec<String>) -> String {
    
    if !keys.contains(&data) {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    let (mut history_clients, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

    if history_parsing_error == utils::ParseError::MalformedToml {
        if let Some(h_client) = history_clients.clients.get("") {
            return h_client.commands[0].clone()
        }
    }

    if let Some(client) = history_clients.clients.get_mut(&data) {
        client.commands.clear();

        let toml_str = toml::to_string(&history_clients).unwrap();
        fs::write(".anthrakas_server_his.toml", &toml_str).unwrap();
        
        let output = format!("({}): Successfully cleared all commands.", colored::Colorize::blue("Success"));
        println!("{}", output);
        return format!("{}\n", output)

    } else {
        
        let output = format!("({}): Client didn't exist.", colored::Colorize::red("Error"));
        println!("{}", output);
        return format!("{}\n", output)
    }
}


