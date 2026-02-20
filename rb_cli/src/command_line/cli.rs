use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    command: String,
    key: String,
}

impl Cli {
    pub fn get_command(&self) -> &String {
        &self.command
    }
}
