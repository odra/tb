use clap::Parser;
use clap::CommandFactory;

#[derive(Parser)]
#[command()]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    command: Option<String>,

    #[arg(allow_hyphen_values(true))]
    args: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    // subcommand was not provided
    // print help as there is nothing to do or run
    if cli.command.is_none() {
        Cli::command().print_long_help().unwrap();

        return;
    }

    // check if tb-$command exists and is executable
    // fail with an error that tb-$command was not found in PATH

    // spawn tb-$command forwarding all args
    // in a new thread, no need for asyncio support (for now)

    println!("Verbose: {:?}", cli.verbose);
    println!("Command: {:?}", cli.command);
    println!("Args: {:?}", cli.args);
}
