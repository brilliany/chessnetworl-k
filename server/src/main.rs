use config::{Config, File};

mod session;
mod server;

fn main() {
    server::main();
}

fn get_config() -> Config {
    let config = Config::builder()
        .add_source(File::with_name("config.yml"))
        .build()
        .expect("Failed to load config");;
    config
}

fn get_config_value(config: &Config, key: &str) -> String {
    let val = config.get::<String>(key).unwrap();
    val
}