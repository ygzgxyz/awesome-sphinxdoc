use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub links: Vec<Link>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error.Error>> {
    let toml_str = fs::read_to_string("links.toml")?;

    let data: Data = toml::from_str(&toml_str)?;
    let items = data.links;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::default())
        .build()
        .unwrap();

    let futures = items.into_iter().map(|item| {
        let client = client.clone();
        tokio::spawn(async move {
            let res = client.get(&item.link).send().await;
            (item.link, res)
        })
    });

    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok((link, Ok(res))) => {
                println!("{}: {}", link, res.status());
            }
            Ok((link, Err(e))) => {
                println!("{}: Error({})", link, e);
            }
            Err(e) => {
                println!("Task failed: {}", e);
            }
        }
    }
    Ok(())
}