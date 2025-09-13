use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::{error, info};

pub async fn scrap_helper(http_client: &Client, url: &str, query: &str) -> Result<Vec<String>> {
    info!("Starting scraping for URL: {}", url);

    let webpage_response = http_client
        .get(url)
        .send()
        .await
        .context("Failed to send GET request")
        .map_err(|e| {
            error!("GET request failed for {}: {:?}", url, e);
            e
        })?;

    info!("Received response from URL: {}", url);

    let webpage = webpage_response
        .text()
        .await
        .context("Failed to read response text")
        .map_err(|e| {
            error!("Failed to read response text for {}: {:?}", url, e);
            e
        })?;

    info!("Parsing HTML document for URL: {}", url);
    let document = Html::parse_document(&webpage);

    let selector = Selector::parse(query).map_err(|e| {
        error!("Failed to parse CSS selector '{}': {:?}", query, e);
        anyhow!("Failed to parse CSS selector: {}", e)
    })?;

    let element = document
        .select(&selector)
        .next()
        .context("No element matched the selector")
        .map_err(|e| {
            error!("No element matched selector '{}'", query);
            e
        })?;

    let element_text_content: Vec<String> = element.text().map(String::from).collect();
    info!(
        "Scraped {} text nodes from selector '{}'",
        element_text_content.len(),
        query
    );

    Ok(element_text_content)
}
