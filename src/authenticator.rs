use crate::utils::scrap_helper;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;
use tracing::info;

pub enum Platforms {
    Codechef,
    Leetcode,
    Codeforces,
}

const LEETCODE_GRAPHQL_BASE: &str = "https://leetcode.com/graphql?query=query";
const CODECHEF_URL_BASE: &str = "https://www.codechef.com/users";
const CODEFORCES_API_URL_BASE: &str = "https://codeforces.com/api";

pub async fn codechef(http_client: &Client, username: &str, code: &str) -> Result<bool> {
    let selector = "title";
    let codechef_profile_page = format!("{CODECHEF_URL_BASE}/{username}");
    info!("Scraping Codechef profile page: {}", codechef_profile_page);

    let scraped_content = scrap_helper(http_client, &codechef_profile_page, selector)
        .await
        .context("Failed to scrape Codechef profile")?;

    for text in &scraped_content {
        for word in text.split_whitespace() {
            if word == code {
                info!("Found code '{}' for user '{}' on Codechef", code, username);
                return Ok(true);
            }
        }
    }

    info!(
        "Code '{}' not found for user '{}' on Codechef",
        code, username
    );
    Ok(false)
}

pub async fn leetcode(http_client: &Client, username: &str, code: &str) -> Result<bool> {
    let graphql_query = format!(
        "{LEETCODE_GRAPHQL_BASE} {{ matchedUser(username: \"{}\") {{ profile {{ realName }} }} }}",
        username
    );
    info!("Querying Leetcode GraphQL for user: {}", username);

    let response = http_client
        .get(&graphql_query)
        .send()
        .await
        .context("Failed to send Leetcode GraphQL request")?;

    let json_response: Value = response
        .json()
        .await
        .context("Failed to parse Leetcode GraphQL response as JSON")?;

    if let Some(real_name) = json_response["data"]["matchedUser"]["profile"]["realName"].as_str() {
        for word in real_name.split_whitespace() {
            if word == code {
                info!("Found code '{}' for user '{}' on Leetcode", code, username);
                return Ok(true);
            }
        }
    }

    info!(
        "Code '{}' not found for user '{}' on Leetcode",
        code, username
    );
    Ok(false)
}

pub async fn codeforces(http_client: &Client, username: &str, code: &str) -> Result<bool> {
    let user_info_endpoint = format!("{CODEFORCES_API_URL_BASE}/user.info?handles={username}");
    info!("Querying Codeforces API for user: {}", username);

    let response = http_client
        .get(&user_info_endpoint)
        .send()
        .await
        .context("Failed to send Codeforces API request")?;

    let json_response: Value = response
        .json()
        .await
        .context("Failed to parse Codeforces API response as JSON")?;

    if let Some(status) = json_response["status"].as_str() {
        if status == "FAILED" {
            info!("Codeforces API request failed for user: {}", username);
            return Ok(false);
        }

        if let Some(user) = json_response["result"]
            .as_array()
            .and_then(|arr| arr.get(0))
        {
            for field in ["firstName", "lastName"] {
                if let Some(name) = user[field].as_str() {
                    for word in name.split_whitespace() {
                        if word == code {
                            info!(
                                "Found code '{}' for user '{}' on Codeforces",
                                code, username
                            );
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }

    info!(
        "Code '{}' not found for user '{}' on Codeforces",
        code, username
    );
    Ok(false)
}

#[derive(Clone)]
pub struct Authenticator {
    http_client: Client,
}

impl Authenticator {
    pub fn new() -> Authenticator {
        Authenticator {
            http_client: Client::new(),
        }
    }

    pub async fn verify(&self, platform: Platforms, username: &str, code: &str) -> Result<bool> {
        match platform {
            Platforms::Leetcode => leetcode(&self.http_client, username, code).await,
            Platforms::Codechef => codechef(&self.http_client, username, code).await,
            Platforms::Codeforces => codeforces(&self.http_client, username, code).await,
        }
    }
}
