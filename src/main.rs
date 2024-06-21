use irc_rust::cli::{self, Commands};
use irc_rust::server;

fn main() {
    let cli = cli::parse();

    match cli.command {
        Commands::Connect(connect_args) => println!("{:?}", connect_args),
        Commands::Host(host_args) => println!("{:?}", host_args),
        Commands::Server(server_args) => {
            server::start_server(server_args).expect("shouldn't error here");
        }
        Commands::List => println!("list command"),
    };
}
