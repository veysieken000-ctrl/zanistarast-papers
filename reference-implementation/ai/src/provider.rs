use zanistarast_core::provider::{ProviderError, ProviderMetadata, ScientificProvider};
use zanistarast_core::ScientificObject;

use crate::NativeAiRuntime;

pub struct NativeAiProvider;

impl NativeAiProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ScientificProvider for NativeAiProvider {
    fn id(&self) -> &'static str {
        "native-ai"
    }

    fn name(&self) -> &'static str {
        "Zanistarast Native AI Provider"
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    fn execute(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError> {
        let mut runtime = NativeAiRuntime::new();
        let result = runtime.execute_scientific_request(object.clone());

        if result.kernel_result.runtime_result.certification.verified {
            Ok(object.clone())
        } else {
            Err(ProviderError::Internal(
                "native AI execution failed certification".to_string(),
            ))
        }
    }

    fn metadata(&self) -> ProviderMetadata {
        let mut metadata = ProviderMetadata::new();
        metadata.insert("type".to_string(), "native-ai-runtime".to_string());
        metadata.insert("deterministic".to_string(), "true".to_string());
        metadata
    }
}

impl Default for NativeAiProvider {
    fn default() -> Self {
        Self::new()
    }
}


