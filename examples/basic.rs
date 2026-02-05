use doi::{CrossrefClient, CrossrefConfig, extract_doi_from_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://doi.org/10.5555/12345678";
    let doi = extract_doi_from_url(url)?;

    let mut config = CrossrefConfig::default();
    config.user_agent = Some("doi-basic-example".to_string());

    let client = CrossrefClient::new(config)?;
    let response = client.fetch_metadata(&doi).await?;

    println!("{}", serde_json::to_string(&response).unwrap());

    Ok(())
}
