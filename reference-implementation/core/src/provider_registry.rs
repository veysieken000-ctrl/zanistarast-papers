use std::collections::HashMap;
use std::sync::Arc;

use crate::provider::{ProviderError, ScientificProvider};
use crate::ScientificObject;

#[derive(Default)]
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn ScientificProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register<P>(&mut self, provider: P)
    where
        P: ScientificProvider + 'static,
    {
        self.providers
            .insert(provider.id().to_string(), Arc::new(provider));
    }

    pub fn execute(
        &self,
        provider_id: &str,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError> {
        let provider = self
            .providers
            .get(provider_id)
            .ok_or_else(|| ProviderError::InvalidInput(format!("provider not found: {provider_id}")))?;

        provider.execute(object)
    }

    pub fn contains(&self, provider_id: &str) -> bool {
        self.providers.contains_key(provider_id)
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    pub fn provider_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.providers.keys().cloned().collect();
        ids.sort();
        ids
    }
}


