use reqwest::{self, header::USER_AGENT, header::CONTENT_TYPE};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Crates {
    crates: Vec<Crate>,
}
#[derive(Debug, Deserialize)]
struct Crate {
    id: String,
    name: String,
    newest_version: String,
    max_stable_version: Option<String>,
    #[serde(rename = "updated_at")]
    updated: String,
    downloads: u64,
    #[serde(default)]
    description: String,
    documentation: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://crates.io/api/v1/crates?&page={param}&per_page=10&sort=downloads",
        param = "1");

    let cl = reqwest::Client::new();
    let crate_list = cl
        .get(&url)
        .header(USER_AGENT, "reqwest")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<Crates>()
        .await?;

    //print!("{:?}", crate_list.crates);

    for ct in crate_list.crates.iter() {
        print!("crate name: {}, crate description: {}", ct.name, ct.description);
    }
    
    Ok(())
}
