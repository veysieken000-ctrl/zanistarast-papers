use zanistarast_ai::openai_provider::OpenAiProvider;
use zanistarast_core::provider::ScientificProvider;

#[test]
fn openai_provider_exposes_expected_metadata() {
    let provider = OpenAiProvider::with_model("gpt-5");

    let metadata = provider.metadata();

    assert_eq!(provider.id(), "openai");
    assert_eq!(
        metadata.get("provider"),
        Some(&"openai".to_string())
    );
    assert_eq!(
        metadata.get("model"),
        Some(&"gpt-5".to_string())
    );
    assert_eq!(
        metadata.get("deterministic_wrapper"),
        Some(&"true".to_string())
    );
}


