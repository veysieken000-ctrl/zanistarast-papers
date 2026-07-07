use chrono::Utc;
use serde_json::json;

use zanistarast_ai::provider::NativeAiProvider;
use zanistarast_core::provider_registry::ProviderRegistry;
use zanistarast_core::{CssId, ScientificObject};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = ProviderRegistry::new();

    registry.register(NativeAiProvider::new());

    let object = ScientificObject {
        css_id: CssId("CSS-EXAMPLE-001".to_string()),
        title: "Provider Example".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "example",
            "claim": "Provider registry execution example."
        }),
    };

    let result = registry.execute("native-ai", &object)?;

    println!("Provider executed successfully.");
    println!("CSS ID: {}", result.css_id.0);
    println!("Title : {}", result.title);

    Ok(())
}


