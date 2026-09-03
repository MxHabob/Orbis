use anyhow::{Result, anyhow};
use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const REPO_OWNER: &str = "orbis-edu";
const REPO_NAME: &str = "orbis";

#[derive(Deserialize, Debug)]
struct GitHubRelease {
    pub tag_name: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub url: String,
}

pub async fn download_latest_release(token: &str) -> Result<()> {
    let client = Client::new();
    let mut req = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            REPO_OWNER, REPO_NAME
        ))
        .header(USER_AGENT, "orbis-cli/1.0")
        .header(ACCEPT, "application/vnd.github.v3+json");

    if !token.is_empty() {
        req = req.header(AUTHORIZATION, format!("Bearer {}", token));
    }

    let res = req.send().await?;

    if !res.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch release from GitHub. Status: {}. Ensure your token is valid if the repo is private.",
            res.status()
        ));
    }

    let release: GitHubRelease = res.json().await?;
    println!(
        "{} Found release: {}",
        "[+]".green(),
        release.tag_name.bold()
    );

    // Find the Windows installer (.msi or .exe or .zip)
    let asset = release
        .assets
        .into_iter()
        .find(|a| a.name.ends_with(".msi") || a.name.ends_with(".zip") || a.name.ends_with(".exe"));

    match asset {
        Some(asset) => {
            println!("{} Downloading asset: {}", "[v]".cyan(), asset.name);
            download_asset(&client, &asset, token).await?;
        }
        None => {
            println!(
                "{} No suitable desktop binary found in the latest release.",
                "[!]".red()
            );
        }
    }

    Ok(())
}

async fn download_asset(client: &Client, asset: &GitHubAsset, token: &str) -> Result<()> {
    let mut req = client
        .get(&asset.url)
        .header(USER_AGENT, "orbis-cli/1.0")
        .header(ACCEPT, "application/octet-stream"); // Required to download the actual file, not JSON

    if !token.is_empty() {
        req = req.header(AUTHORIZATION, format!("Bearer {}", token));
    }

    let res = req.send().await?;

    if !res.status().is_success() {
        return Err(anyhow!(
            "Failed to download asset. Status: {}",
            res.status()
        ));
    }

    let total_size = res.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let file_path = Path::new(&asset.name);
    let mut file = File::create(file_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message("Download complete");
    println!(
        "{} Saved to: {:?}",
        "[+]".green(),
        file_path.canonicalize()?
    );

    Ok(())
}
