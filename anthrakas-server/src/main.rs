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
    let master_key = conf.master_key;
    let master_key_two = master_key.clone();
    let master_key_three = master_key.clone();
    
    let app = Router::new()
        .route("/add_command", post(move |data| add_command(data, master_key.clone())))
        .route("/clear_commands", post(move |data| clear_commands(data, master_key_two.clone())))
        .route("/commands", post(move |data| commands(data, master_key_three.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn add_command(data: String, key: String) -> String {
    let (commands_and_key, parsing_error) = utils::parse_post_toml(&data);

    if parsing_error == utils::ParseError::MalformedToml {
        return commands_and_key.commands[0].clone()
    }
    
    if commands_and_key.key != key {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    if !std::path::Path::exists(std::path::Path::new(".anthrakas_server_his.toml")) {
        fs::write(".anthrakas_server_his.toml", "key = 'cat'\ncommands = []").unwrap();
    }
    
    let (mut history_commands, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

    if history_parsing_error == utils::ParseError::MalformedToml {
        return history_commands.commands[0].clone()
    }
    
    history_commands.commands.extend(commands_and_key.commands.into_iter());

    let toml_str = toml::to_string(&history_commands).unwrap();
    fs::write(".anthrakas_server_his.toml", &toml_str).unwrap();


    let output = format!("({}): Successfully added new command.", colored::Colorize::blue("Success"));
    println!("{}", output);
    format!("{}\n", output)
}

async fn commands(data: String, key: String) -> String {
    
    if data != key {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    let (history_commands, history_parsing_error) = utils::parse_commands_history_file(&utils::get_files_contents(".anthrakas_server_his.toml"));

    if history_parsing_error == utils::ParseError::MalformedToml {
        return history_commands.commands[0].clone()
    }

    println!("({}): Successfully displayed all commands.", colored::Colorize::blue("Success"));

    toml::to_string(&history_commands).unwrap().lines().last().unwrap().to_string()
}

async fn clear_commands(data: String, key: String) -> String {
    
    if data != key {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    fs::write(".anthrakas_server_his.toml", "key = 'cat'\ncommands = []").unwrap();

    

    let output = format!("({}): Successfully cleared all commands.", colored::Colorize::blue("Success"));
    println!("{}", output);
    format!("{}\n", output)
}


