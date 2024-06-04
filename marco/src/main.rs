// A command line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selman Karaosmanoglu",
    about = "Marco Polo Game"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selman Karaosmanoglu")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command given"),
    }
}
