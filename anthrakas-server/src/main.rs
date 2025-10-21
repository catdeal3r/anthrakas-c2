use axum::{
    routing::{get, post},
    Router,
};
use toml;
use std::fs::File;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/add_command", post(send_command))
        .route("/commands", get(commands));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    "".to_string()
}

async fn send_command(payload: String) -> String {
    let commands: Commands = toml::from_str(&payload).unwrap();
    // let mut current_commands_toml: String = toml::from_str(fs::read_to_string(".command_cache").unwrap()).unwrap();

    // current_commands_toml.commands.push_str();
    
    
    toml::to_string(&commands).unwrap()
}

async fn commands() -> String {
    "{\"id\":1337,\"username\":\"banger\"}".to_string()
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
struct Commands {
   key: String,
   commands: Vec<String>,
}
