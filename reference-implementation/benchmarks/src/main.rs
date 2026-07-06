use chrono::Utc;
use serde_json::json;
use std::time::Instant;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_runtime::DeterministicRuntime;

fn main() {
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-BENCH-001".to_string()),
        title: "Benchmark Scientific Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "benchmark_claim",
            "claim": "Benchmark object for deterministic runtime verification.",
            "layers": {
                "hebun": "existence verified",
                "zanabun": "meaning verified",
                "mabun": "structure verified",
                "rabun": "operation verified",
                "rasterast": "final verification completed"
            }
        }),
    };

    let mut runtime = DeterministicRuntime::new();

    let start = Instant::now();
    let result = runtime.execute(object);
    let elapsed = start.elapsed();

    println!("ZANISTARAST RUNTIME BENCHMARK");
    println!("-----------------------------");
    println!("Verification Passed: {}", result.verification.passed);
    println!("Certification Verified: {}", result.certification.verified);
    println!("Published: {}", result.publication.is_some());
    println!("Elapsed: {:?}", elapsed);
}





