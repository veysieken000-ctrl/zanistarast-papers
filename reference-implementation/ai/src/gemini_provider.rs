zanistarast_core::provider::{ProviderError, ProviderMetadata, ScientificProvider} kullanın;
zanistarast_core::ScientificObject kullanın;

pub struct GeminiProvider;

impl GeminiProvider {
pub fn new() -> Self {
Kendi
}
}

GeminiProvider için ScientificProvider'ı uygulayın {
fn id(&self) -> &'static str {
"ikizler burcu"
}

fn name(&self) -> &'static str {
"Zanistarast İkizler Burcu Sağlayıcısı"
}

fn version(&self) -> &'static str {
"0.1.0"
}

fn yürüt(
&kendisi,
nesne: &BilimselNesne,
) -> Sonuç<BilimselNesne, SağlayıcıHatası> {
Tamam(nesne.klon())
}

fn metadata(&self) -> ProviderMetadata {
let mut metadata = ProviderMetadata::new();
metadata.insert("type".to_string(), "cloud-ai-provider".to_string());
metadata.insert("provider".to_string(), "gemini".to_string());
metadata.insert("deterministic_wrapper".to_string(), "true".to_string());
metadata.insert("api_enabled".to_string(), "false".to_string());
meta veriler
}
}

GeminiProvider için varsayılan değer uygulanmıştır.
fn default() -> Self {
Kendi::yeni()
}
}


