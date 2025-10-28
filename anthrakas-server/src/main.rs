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

    let keys = conf.keys;
    let keys_two = keys.clone();
    let keys_three = keys.clone();
    let keys_four = keys.clone();
    let keys_five = keys.clone();
    let keys_six = keys.clone();
    
    let app = Router::new()
        .route("/add_commands", post(move |data| commands::add_commands(data, keys.clone())))
        .route("/clear_commands", post(move |data| commands::clear_commands(data, keys_two.clone())))
        .route("/commands", post(move |data| commands::commands(data, keys_three.clone())))
        .route("/add_outputs", post(move |data| outputs::add_outputs(data, keys_four.clone())))
        .route("/clear_outputs", post(move |data| outputs::clear_outputs(data, keys_five.clone())))
        .route("/outputs", post(move |data| outputs::outputs(data, keys_six.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

