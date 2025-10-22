use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    clients: HashMap<String, Section>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Section {
    commands: Vec<String>,
}

fn main() {
    let toml_data = r#"
        [clients.CoolKeyName]
        commands = ["info", "info", "info"]

        [clients.AnotherKey]
        commands = []
    "#;

    // Deserialize the TOML data into the Config struct
    let mut config: Config = toml::de::from_str(toml_data).unwrap();
    config.clients.insert("monke".to_string(), Section { commands: vec!["monke data".to_string(), "monke data again".to_string()] });

    // Print the deserialized data
    println!("{:?}", config);

    let toml_config_string = toml::ser::to_string(&config).unwrap();
    println!("{}", toml_config_string);
}
