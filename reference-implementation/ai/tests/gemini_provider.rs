use zanistarast_ai::gemini_provider::GeminiProvider;
use zanistarast_core::provider::ScientificProvider;

#[test]
fn gemini_provider_metadata() {
    let provider = GeminiProvider::with_model("gemini-test-model");

    assert_eq!(provider.id(), "gemini");
    assert_eq!(provider.name(), "Zanistarast Gemini Provider");
    assert_eq!(provider.version(), "0.1.0");

    let metadata = provider.metadata();

    assert_eq!(metadata.get("provider").unwrap(), "gemini");
    assert_eq!(metadata.get("model").unwrap(), "gemini-test-model");
    assert_eq!(
        metadata.get("deterministic_wrapper").unwrap(),
        "true"
    );
}


