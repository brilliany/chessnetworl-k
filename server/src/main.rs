use config::{Config, File, FileFormat};

mod server;
mod session;

fn main() {
    let config = get_config();

    if let Err(e) = server::main(config) {
        eprintln!("Server error: {}", e);
    }
}

fn get_config() -> Config {
    let config = Config::builder()
        .add_source(File::with_name("config.yml"))
        .build()
        .expect("Failed to load config");;
    config
}

fn get_config_value(config: &Config, key: &str) -> String {
    let mode = config.get::<String>(key).unwrap();
    mode
}