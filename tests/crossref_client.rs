use doi::{CrossrefClient, CrossrefConfig, Doi};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

struct SequenceResponder {
    responses: Vec<ResponseTemplate>,
    counter: Arc<AtomicUsize>,
}

impl Respond for SequenceResponder {
    /// Returns the next response in sequence, repeating the last when exhausted.
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let index = self.counter.fetch_add(1, Ordering::SeqCst);
        self.responses
            .get(index)
            .cloned()
            .unwrap_or_else(|| self.responses.last().cloned().unwrap())
    }
}

/// Build a CrossrefConfig pointing to the mock server.
fn config_for_server(server: &MockServer) -> CrossrefConfig {
    CrossrefConfig {
        base_url: server.uri(),
        ..CrossrefConfig::default()
    }
}

/// Return a stable DOI value for tests.
fn example_doi() -> Doi {
    Doi {
        value: "10.5555/abc".to_string(),
    }
}

/// Create a successful Crossref response payload.
fn example_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(
        r#"{"status":"ok","message-type":"work","message-version":"1.0.0","message":{"title":["Example"]}}"#,
        "application/json",
    )
}

#[tokio::test]
/// Retries on HTTP 429 and succeeds on the next response.
async fn crossref_client_retry_on_429() {
    let server = MockServer::start().await;

    let response_429 = ResponseTemplate::new(429)
        .append_header("Retry-After", "0")
        .set_body_raw("too many", "text/plain");
    let responder = SequenceResponder {
        responses: vec![response_429, example_response()],
        counter: Arc::new(AtomicUsize::new(0)),
    };

    Mock::given(method("GET"))
        .and(path("/works/10.5555/abc"))
        .respond_with(responder)
        .expect(2)
        .mount(&server)
        .await;

    let client = CrossrefClient::new(config_for_server(&server)).expect("client");
    let response = client.metadata(&example_doi()).await.expect("response");

    assert_eq!(response.message.title, vec!["Example".to_string()]);
}

#[tokio::test]
/// Does not retry on HTTP 404 responses.
async fn crossref_client_no_retry_on_404() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/works/10.5555/abc"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;

    let client = CrossrefClient::new(config_for_server(&server)).expect("client");
    let result = client.metadata(&example_doi()).await;

    assert!(result.is_err());
}

#[tokio::test]
/// Sends a user-agent header when configured.
async fn crossref_client_user_agent_header() {
    let server = MockServer::start().await;

    let mailto = "test@example.com";
    let expected_user_agent = format!("{} mailto:{}", "TestApp", mailto);

    Mock::given(method("GET"))
        .and(path("/works/10.5555/abc"))
        .and(header("user-agent", expected_user_agent.as_str()))
        .respond_with(example_response())
        .expect(1)
        .mount(&server)
        .await;

    let mut config = config_for_server(&server);
    config.user_agent = Some("TestApp".to_string());
    config.mailto = Some(mailto.to_string());
    let client = CrossrefClient::new(config).expect("client");

    client.metadata(&example_doi()).await.expect("response");
}

#[tokio::test]
/// Includes the mailto in the User-Agent header when configured.
async fn crossref_client_mailto_query_param() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/works/10.5555/abc"))
        .respond_with(example_response())
        .expect(1)
        .mount(&server)
        .await;

    let mut config = config_for_server(&server);
    config.mailto = Some("test@example.com".to_string());
    let client = CrossrefClient::new(config).expect("client");
    client.metadata(&example_doi()).await.expect("response");

    let received = server.received_requests().await.unwrap();
    let request = received.first().expect("request");
    let mailto = request
        .url
        .query_pairs()
        .find(|(key, _)| key == "mailto")
        .map(|(_, value)| value.to_string());

    assert!(mailto.is_none());
    let user_agent = request
        .headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok());
    assert_eq!(user_agent, Some("mailto:test@example.com"));
}

#[tokio::test]
/// Omits mailto and user-agent when not configured.
async fn crossref_client_no_mailto_query_param() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/works/10.5555/abc"))
        .respond_with(example_response())
        .expect(1)
        .mount(&server)
        .await;

    let client = CrossrefClient::new(config_for_server(&server)).expect("client");
    client.metadata(&example_doi()).await.expect("response");

    let received = server.received_requests().await.unwrap();
    let request = received.first().expect("request");
    let mailto = request
        .url
        .query_pairs()
        .find(|(key, _)| key == "mailto")
        .map(|(_, value)| value.to_string());

    assert!(mailto.is_none());
    assert!(request.headers.get("user-agent").is_none());
}
