use doi::crossref::models::CrossrefResponse;

#[test]
fn crossref_models_deserialize_fixture() {
    let payload = std::fs::read_to_string("tests/fixtures/crossref.json")
        .expect("fixture should be readable");
    let response: CrossrefResponse =
        serde_json::from_str(&payload).expect("fixture should deserialize");

    assert_eq!(response.status, "ok");
    assert_eq!(response.message_type, "work");
    assert_eq!(response.message.doi.as_deref(), Some("10.5555/12345678"));
}
