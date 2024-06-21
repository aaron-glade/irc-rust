use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(name = "irc")]
#[command(version = "1.0")]
#[command(about = "Internet Relay Chat implemented in Rust using TCP sockets.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    Connect(ConnectArgs),
    Host(HostArgs),
    Server(ServerArgs),
    List,
}

#[derive(Args, Debug)]
pub struct ConnectArgs {
    pub address: String,
    pub port: String,
}

#[derive(Args, Debug)]
pub struct HostArgs {
    pub title: String,
    pub description: Option<String>,
    pub host_name: Option<String>,
}

#[derive(Args, Debug)]
pub struct ServerArgs {
    pub address: Option<String>, // Defaults to 0.0.0.0
    pub port: Option<String>, // Defaults to 8080
}

pub fn parse() -> Cli {
    Cli::parse()    
}

