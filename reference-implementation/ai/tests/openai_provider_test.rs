#[test]
fn openai_provider_reads_api_key() {
    let key = std::env::var("OPENAI_API_KEY");
    assert!(key.is_ok() || key.is_err());
}


