use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(long, env, default_value = "3000")]
    pub server_port: String,

    #[clap(long, env, default_value = "0.0.0.0")]
    pub server_host: String,

    #[clap(long, env)]
    pub database_url: String,
}
