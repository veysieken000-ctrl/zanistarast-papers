use chrono::Utc;
use serde_json::json;

use zanistarast_ai::NativeAiRuntime;
use zanistarast_core::{CssId, ScientificObject};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = NativeAiRuntime::new();

    let object = ScientificObject {
        css_id: CssId("CSS-END-TO-END-001".to_string()),
        title: "End-to-End Example".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "Complete certified execution example."
        }),
    };

    let result = runtime.execute_scientific_request(object);

    println!("==============================");
    println!("ZANISTARAST END-TO-END EXAMPLE");
    println!("==============================");

    println!("AI Session : {:?}", result.ai_session_id);
    println!("Kernel : {:?}", result.kernel_result.kernel_id);
    println!("Runtime : {:?}", result.kernel_result.runtime_result.runtime_id);

    println!(
        "Verification : {}",
        result.kernel_result.runtime_result.verification.passed
    );

    println!(
        "Certification : {}",
        result.kernel_result.runtime_result.certification.verified
    );

    println!(
        "Published : {}",
        result.kernel_result.runtime_result.publication.is_some()
    );

    Ok(())
}


