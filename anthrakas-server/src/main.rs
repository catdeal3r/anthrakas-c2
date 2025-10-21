use axum::{
    routing::post,
    Router,
};
use toml;

pub mod utils;

#[tokio::main]
async fn main() {
    let conf: utils::Config = utils::parse_config(&utils::get_config());
    let master_key = conf.master_key;
    let master_key_two = master_key.clone();
    
    let app = Router::new()
        .route("/add_command", post(move |data| send_command(data, master_key.clone())))
        .route("/commands", post(move |data| commands(data, master_key_two.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn send_command(data: String, key: String) -> String {
    let (commands_and_key, parsing_error) = utils::parse_post_toml(&data);

    if parsing_error == utils::ParseError::MalformedToml {
        return commands_and_key.commands[0].clone()
    }
    
    if commands_and_key.key != key {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }
    
    toml::to_string(&commands_and_key).unwrap()
}

async fn commands(data: String, key: String) -> String {
    let (commands_and_key, parsing_error) = utils::parse_post_toml(&data);

    if parsing_error == utils::ParseError::MalformedToml {
        return commands_and_key.commands[0].clone()
    }

    if commands_and_key.key != key {
        let error = format!("({}): Incorrect authentication.", colored::Colorize::red("Error"));
        println!("{}", error);
        return format!("{}\n", error)
    }

    "Dummy commands".to_string()
}



