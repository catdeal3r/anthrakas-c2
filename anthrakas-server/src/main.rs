use axum::{
    routing::post,
    Router,
};

pub mod utils;
pub mod commands;
pub mod outputs;

#[tokio::main]
async fn main() {
    let conf: utils::Config = utils::parse_config(&utils::get_files_contents("anthrakas_server_conf.toml"));

    let clients = conf.clients;
    let clients_two = clients.clone();
    let clients_three = clients.clone();
    let clients_four = clients.clone();
    let clients_five = clients.clone();
    let clients_six = clients.clone();
    
    let app = Router::new()
        .route("/add_commands", post(move |data| commands::add_commands(data, clients.clone())))
        .route("/clear_commands", post(move |data| commands::clear_commands(data, clients_two.clone())))
        .route("/commands", post(move |data| commands::commands(data, clients_three.clone())))
        .route("/add_outputs", post(move |data| outputs::add_outputs(data, clients_four.clone())))
        .route("/clear_outputs", post(move |data| outputs::clear_outputs(data, clients_five.clone())))
        .route("/outputs", post(move |data| outputs::outputs(data, clients_six.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

