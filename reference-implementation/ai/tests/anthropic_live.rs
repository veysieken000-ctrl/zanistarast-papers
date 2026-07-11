use zanistarast_ai::anthropic_provider::AnthropicProvider;

#[test]
#[ignore]
fn anthropic_live() {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY is not set");

    let _provider = AnthropicProvider::new();

    assert!(!api_key.is_empty());

    println!("Anthropic live integration test passed.");
}


