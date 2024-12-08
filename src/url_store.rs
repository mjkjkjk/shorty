use crate::shortener::UrlEntry;
use anyhow::{Context, Result};
use rusqlite::{params, Connection};

pub struct UrlStore {
    conn: Connection,
}

impl UrlStore {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("urls.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS urls (
                short_code TEXT PRIMARY KEY,
                original_url TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                expires_at DATETIME DEFAULT NULL,
                clicks INTEGER DEFAULT 0
            )",
            [],
        )?;

        Ok(UrlStore { conn })
    }

    pub fn create_url(&self, short_code: &str, original_url: &str, ttl: Option<u32>) -> Result<()> {
        if ttl.is_some() {
            self.conn.execute(
                "INSERT INTO urls (short_code, original_url, expires_at) 
                 VALUES (?1, ?2, datetime(CURRENT_TIMESTAMP, '+' || ?3 || ' seconds'))",
                params![short_code, original_url, ttl.unwrap()],
            )?;
        } else {
            self.conn.execute(
                "INSERT INTO urls (short_code, original_url) VALUES (?1, ?2)",
                params![short_code, original_url],
            )?;
        }

        Ok(())
    }

    pub fn get_url(&self, short_code: &str) -> Result<String> {
        let url = self
            .conn
            .query_row(
                "SELECT original_url FROM urls WHERE short_code = ?1",
                params![short_code],
                |row| row.get(0),
            )
            .context("URL not found")?;

        // Increment click count
        self.conn.execute(
            "UPDATE urls SET clicks = clicks + 1 WHERE short_code = ?1",
            params![short_code],
        )?;

        Ok(url)
    }

    pub fn list_urls(&self) -> Result<Vec<UrlEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT short_code, original_url, created_at, clicks, expires_at FROM urls")?;

        let url_iter = stmt.query_map([], |row| {
            Ok(UrlEntry {
                short_code: row.get(0)?,
                original_url: row.get(1)?,
                created_at: row.get(2)?,
                clicks: row.get(3)?,
                expires_at: row.get(4)?,
            })
        })?;

        let mut urls = Vec::new();
        for url in url_iter {
            urls.push(url?);
        }

        Ok(urls)
    }
}
