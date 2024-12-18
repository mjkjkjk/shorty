use crate::url_store::UrlStore;
use anyhow::Result;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct UrlEntry {
    pub short_code: String,
    pub original_url: String,
    pub created_at: String,
    pub clicks: u32,
    pub expires_at: String,
}

pub fn create_short_url(original_url: &str, desired_short_code: Option<String>) -> Result<String> {
    let url = Url::parse(original_url);
    if url.is_err() {
        return Err(anyhow::anyhow!("Invalid URL"));
    }

    let short_code = desired_short_code.unwrap_or(generate_short_code());

    let ttl = 24 * 60 * 60; // 1 day
    let store = UrlStore::new()?;
    store.create_url(&short_code, original_url, Some(ttl))?;

    Ok(short_code)
}

pub fn retrieve_url(short_code: &str) -> Result<String> {
    let store = UrlStore::new()?;
    store.get_url(short_code)
}

pub fn list_urls() -> Result<Vec<UrlEntry>> {
    let store = UrlStore::new()?;
    store.list_urls()
}

fn generate_short_code() -> String {
    Uuid::new_v4().to_string()[..8].to_string()
}
