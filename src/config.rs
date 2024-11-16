use std::env;

use once_cell::sync::Lazy;

use crate::tracing::error;

pub struct Config {
    pub port: String,
    pub rabbitmq_host: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let port = get_env("PORT");
    let rabbitmq_host = get_env("RABBITMQ_HOST");
    let rabbitmq_port = get_env("RABBITMQ_PORT").parse().unwrap();
    let rabbitmq_user = get_env("RABBITMQ_USER");
    let rabbitmq_password = get_env("RABBITMQ_PASSWORD");

    Config {
        port,
        rabbitmq_host,
        rabbitmq_port,
        rabbitmq_user,
        rabbitmq_password,
    }
});

pub fn get_config() -> &'static Config {
    &CONFIG
}

fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_err| {
        error!("{} is not set", name);
        std::process::exit(1);
    })
}
