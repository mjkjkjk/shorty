use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        url: String,
        short_code: Option<String>,
    },
    Retrieve {
        short_code: String,
    },
    Serve {
        port: Option<u16>,
    },
    List,
    Info,
}
