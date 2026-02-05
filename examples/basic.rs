use doi::{CrossrefClient, CrossrefConfig, extract_doi_from_url};

#[tokio::main]
/// Demonstrates DOI extraction and Crossref metadata retrieval.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://doi.org/10.5555/12345678";
    let doi = extract_doi_from_url(url).ok_or("doi not found")?;

    let config = CrossrefConfig {
        user_agent: Some("doi-basic-example".to_string()),
        mailto: Some("me@example.com".to_string()),
        ..Default::default()
    };

    let client = CrossrefClient::new(config)?;
    let response = client.metadata(&doi).await?;

    println!("{}", serde_json::to_string(&response).unwrap());
    // Show the extracted DOI string for reference.
    println!("DOI: {}", doi.as_str());

    Ok(())
}
