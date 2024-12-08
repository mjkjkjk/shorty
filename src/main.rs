mod cli;
mod shortener;
mod url_store;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => match cmd {
            cli::Commands::Create { url } => {
                let short_url = shortener::create_short_url(&url)?;
                println!("Created short URL: {}", short_url);
            }
            cli::Commands::Retrieve { short_code } => {
                let original_url = shortener::retrieve_url(&short_code)?;
                println!("Original URL: {}", original_url);
            }
            cli::Commands::List => {
                let urls = shortener::list_urls()?;
                for url in urls {
                    println!("{}: {}", url.short_code, url.original_url);
                }
            }
        },
        None => {
            // TODO add actual help
            println!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}
