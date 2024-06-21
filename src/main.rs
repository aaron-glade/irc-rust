use irc_rust::cli::{self, Commands};

fn main() {
    let cli = cli::parse();

    match cli.command {
        Commands::Connect(connect_args) => println!("{:?}", connect_args),
        Commands::Host(host_args) => println!("{:?}", host_args),
        Commands::List => println!("list command"),
    };
}
