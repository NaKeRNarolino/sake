use std::path::Path;
use reqwest::Client;

pub async fn init() {
    let dir = std::env::current_dir().unwrap();

    let project_dir = dir.clone();
    let mut dotsake_dir = dir.clone();
    dotsake_dir.push("./sake");

    log::info!("Downloading Sake...");

    download_file_async().await.unwrap()
}

async fn download_file_async(url: &str, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }

    let mut file = tokio::fs::File::create(destination).await?;
    let mut content = response.bytes_stream();

    while let Some(item) = content.next().await {
        tokio::io::copy(&mut item?.as_ref(), &mut file).await?;
    }

    Ok(())
}