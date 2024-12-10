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
            cli::Commands::Create { url, short_code } => {
                let short_url = shortener::create_short_url(&url, short_code)?;
                println!("Created short URL: {}", short_url);
            }
            cli::Commands::Retrieve { short_code } => {
                let original_url = shortener::retrieve_url(&short_code)?;
                println!("Original URL: {}", original_url);
            }
            cli::Commands::List => {
                let urls = shortener::list_urls()?;
                for url in urls {
                    println!(
                        "{}: {} (expires at {})",
                        url.short_code, url.original_url, url.expires_at
                    );
                }
            }
            cli::Commands::Info => {
                println!("Available commands:");
                println!("  create <url> [short_code] - Create a short URL");
                println!("  retrieve <short_code> - Retrieve the original URL");
                println!("  list - List all short URLs");
                println!("  info - Show this help message");
            }
        },
        None => {
            println!("No command specified. Use 'info' for usage information.");
        }
    }

    Ok(())
}
