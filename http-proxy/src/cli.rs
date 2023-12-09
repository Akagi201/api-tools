use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[clap(long, value_name = "PORT", default_value = "8088")]
    pub port: u16,
    #[clap(long, value_name = "HOST", default_value = "0.0.0.0")]
    pub host: String,
}
