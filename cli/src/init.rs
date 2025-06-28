use std::fs;
use std::fs::create_dir_all;
use futures_util::StreamExt;
use std::path::Path;
use reqwest::Client;

use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::new("templates/**/*").unwrap();
        
        tera.add_raw_templates(vec![
            ("dotenv", include_str!("../templates/dotenv.tmpl")),
            ("config", include_str!("../templates/config.tmpl")),
            ("projectjson", include_str!("../templates/project.json"))
        ]).unwrap();
        
        tera
    };
}

pub async fn init() {
    let dir = std::env::current_dir().unwrap();

    let project_dir = dir.clone();
    let mut dotsake_dir = dir.clone();
    dotsake_dir.push("./.sake");

    create_dir_all(&dotsake_dir).unwrap();

    log::info!("Downloading Sake...");

    download_file_async("https://github.com/NaKeRNarolino/sake/releases/download/0.1.0-linux/sake", &project_dir.join("sake")).await.unwrap();

    let mut c = tera::Context::new();

    c.insert("project", project_dir.to_str().unwrap());
    c.insert("dotsake", dotsake_dir.to_str().unwrap());

    fs::write(&project_dir.join(".env"),
        TERA.render("dotenv", &c).unwrap()).unwrap();
    
    fs::write(&project_dir.join("config.json"),
              TERA.render("config", &Context::new()).unwrap()).unwrap();

    fs::write(&project_dir.join("project.json"),
              TERA.render("projectjson", &Context::new()).unwrap()).unwrap();
    
    create_dir_all(&project_dir.join("src/BP")).unwrap();
    create_dir_all(&project_dir.join("src/RP")).unwrap();
    
    log::info!("Project created. Don't forget to set the com.mojang directory in .env! Happy coding ^.^")
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