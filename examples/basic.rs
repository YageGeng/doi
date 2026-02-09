use doi::{DoiOrgClient, DoiOrgConfig};

#[tokio::main]
/// Demonstrates DOI extraction and Crossref metadata retrieval.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doi = "https://doi.org/10.5555/12345678".parse()?;

    let config = DoiOrgConfig {
        user_agent: Some("doi-basic-example".to_string()),
        mailto: Some("me@example.com".to_string()),
        ..Default::default()
    };

    let client = DoiOrgClient::new(config)?;
    let response = client.metadata(&doi).await?;

    println!("{}", serde_json::to_string(&response).unwrap());
    println!("DOI: {}", doi.as_str());

    let arxiv_doi = "https://arxiv.org/abs/2512.06879".parse()?;
    let arxiv_response = client.metadata(&arxiv_doi).await?;
    println!("{}", serde_json::to_string(&arxiv_response).unwrap());
    // Show the derived arXiv DOI string for reference.
    println!("arXiv DOI: {}", arxiv_doi.as_str());

    Ok(())
}
