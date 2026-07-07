use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use tracing::info;
use zanistarast_ai::NativeAiRuntime;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_core::config_loader::load_config;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
let config = load_config("config/runtime.toml")?;
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-001".to_string()),
        title: "First Certified Scientific Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "A scientific object must pass deterministic verification before certification.",
            "layers": {
                "hebun": "existence verified",
                "zanabun": "meaning verified",
                "mabun": "structure verified",
                "rabun": "operation verified",
                "rasterast": "final verification required"
            }
        }),
    };

    let mut ai_runtime = NativeAiRuntime::new();
    let result = ai_runtime.execute_scientific_request(object);

    let runtime_result = &result.kernel_result.runtime_result;

    info!("AI Session ID: {:?}", result.ai_session_id);
    info!("Kernel ID: {:?}", result.kernel_result.kernel_id);
    info!("Runtime ID: {:?}", runtime_result.runtime_id);
    info!("Verification passed: {}", runtime_result.verification.passed);
    info!("Certification verified: {}", runtime_result.certification.verified);

    println!("ZANISTARAST END-TO-END CERTIFIED EXECUTION");
    println!("----------------------------------------");
    println!("AI Session: {:?}", result.ai_session_id);
    println!("Kernel: {:?}", result.kernel_result.kernel_id);
    println!("Runtime: {:?}", runtime_result.runtime_id);
    println!("Verification Passed: {}", runtime_result.verification.passed);
    println!("Certification Verified: {}", runtime_result.certification.verified);
    println!("Default Provider: {}", config.provider.default_provider);
    println!("Logging Level: {}", config.logging.level);
    
    match &runtime_result.publication {
        Some(publication) => {
            info!("Registry publication: {:?}", publication.registry_id);
            println!("Registry ID: {:?}", publication.registry_id);
            println!("Publication Status: Published");
        }
        None => {
            info!("Registry publication skipped");
            println!("Registry ID: None");
            println!("Publication Status: Skipped");
        }
    }

    Ok(())
}


