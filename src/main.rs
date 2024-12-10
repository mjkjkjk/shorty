mod cli;
mod shortener;
mod url_store;

use anyhow::Result;
use axum::{extract::Path, response::Redirect, routing::get, Router};
use clap::Parser;
use cli::Cli;
use tower_http::trace::TraceLayer;

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
                println!("  serve - Start the web server");
                println!("  info - Show this help message");
            }
            cli::Commands::Serve { port } => {
                let app = Router::new()
                    .route("/:short_code", get(handle_redirect))
                    .route("/", get(handle_root))
                    .layer(TraceLayer::new_for_http());

                let listener =
                    tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port.unwrap_or(3000)))
                        .await
                        .unwrap();
                axum::serve(listener, app).await.unwrap();
            }
        },
        None => {
            println!("No command specified. Use 'info' for usage information.");
        }
    }

    Ok(())
}

async fn handle_redirect(
    Path(short_code): Path<String>,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    match shortener::retrieve_url(&short_code) {
        Ok(original_url) => Ok(Redirect::permanent(&original_url)),
        Err(_) => Err((
            axum::http::StatusCode::NOT_FOUND,
            "Short URL not found".to_string(),
        )),
    }
}

async fn handle_root() -> &'static str {
    "Welcome to URL Shortener. Use /{short_code} to access a shortened URL."
}
