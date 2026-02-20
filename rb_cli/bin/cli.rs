use clap::Parser;
use rb_cli::command_line::cli::Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    println!("Command line: {}", args.get_command());

}