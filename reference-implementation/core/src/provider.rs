use std::collections::HashMap;

use crate::ScientificObject;

pub type ProviderMetadata = HashMap<String, String>;

pub trait ScientificProvider: Send + Sync {
    fn id(&self) -> &'static str;

    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn execute(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError>;

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


