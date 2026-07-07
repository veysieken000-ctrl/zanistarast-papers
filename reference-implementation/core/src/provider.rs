use std::collections::HashMap;

use crate::ScientificObject;

pub type ProviderMetadata = HashMap<String, String>;

pub trait ScientificProvider: Send + Sync {
    /// Unique provider identifier.
    fn id(&self) -> &'static str;

    /// Human readable provider name.
    fn name(&self) -> &'static str;

    /// Provider version.
    fn version(&self) -> &'static str;

    /// Execute a scientific request.
    fn execute(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError>;

    /// Provider metadata.
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata::new()
    }
}

#[derive(Debug)]
pub enum ProviderError {
    Unsupported,
    InvalidInput(String),
    Internal(String),
}


