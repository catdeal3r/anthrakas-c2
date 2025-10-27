use axum::{
    routing::post,
    Router,
};
use toml;
use std::fs;

pub mod utils;

#[tokio::main]
async fn main() {
    let conf: utils::Config = utils::parse_config(&utils::get_files_contents("anthrakas_server_conf.toml"));

    let keys = conf.keys;
    let keys_two = keys.clone();
    let keys_three = keys.clone();
    
    let app = Router::new()
        .route("/add_command", post(move |data| add_command(data, keys.clone())))
        .route("/clear_commands", post(move |data| clear_commands(data, keys_two.clone())))
        .route("/commands", post(move |data| commands(data, keys_three.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn add_command(data: String, keys: Vec<String>) -> String {
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

        if !std::path::Path::exists(std::path::Path::new(".anthrakas_server_his.toml")) {
            let content = format!("[clients.{}]\ncommands = []", key);
            fs::write(".anthrakas_server_his.toml", &content).unwrap();
        }
    
        let (mut history_clients, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

        if history_parsing_error == utils::ParseError::MalformedToml {
            if let Some(h_client) = history_clients.clients.get("") {
                return h_client.commands[0].clone()
            }
        }

        if !history_clients.clients.contains_key(&key.clone()) {
            history_clients.clients.insert(key.clone(), utils::Client { commands: client.commands.clone() } );
        } else {
            if let Some(h_client) = history_clients.clients.get_mut(&key.clone()) {
                h_client.commands.extend(client.commands.clone().into_iter());
            }
        }
        
        let toml_str = toml::to_string(&history_clients).unwrap();
        fs::write(".anthrakas_server_his.toml", &toml_str).unwrap();
    }

    let output = format!("({}): Successfully added new command.", colored::Colorize::blue("Success"));
    println!("{}", output);
    format!("{}\n", output)
}

async fn commands(data: String, keys: Vec<String>) -> String {
    
    if !keys.contains(&data) {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    let (history_clients, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

    if history_parsing_error == utils::ParseError::MalformedToml {
        if let Some(h_client) = history_clients.clients.get("") {
            return h_client.commands[0].clone()
        }
    }

    if let Some(commands) = history_clients.clients.get(&data) {
        println!("({}): Successfully displayed all commands.", colored::Colorize::blue("Success"));
        return toml::to_string(&commands).unwrap()
    } else {
        return "commands = []\n".to_string()
    }
}

async fn clear_commands(data: String, keys: Vec<String>) -> String {
    
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


