serde_json::json kullanın;

zanistarast_core::provider::{ kullanın
    Sağlayıcı Hatası,
    SağlayıcıMetaverileri,
    Bilimsel Sağlayıcı,
};
zanistarast_core:: ScientificObject kullanın ;

`use crate::api_key_manager:: ApiKeyManager;`
`use crate::gemini_responses:: GeminiGenerateClient;`

pub struct GeminiProvider {
    model: Dize,
}

impl GeminiProvider {
    pub fn new() -> Self {
        let model = std::env::var("GEMINI_MODEL")
            .unwrap_or_else(|_| "gemini-2.5-flash".to_string() );

        Kendi modeli
    }

    pub fn with_model(model: impl Into<String>) -> Self {
        Kendim {
            model: model.into(),
        }
    }

    pub async fn generate(
        &kendisi,
        istem: &str,
    ) -> Sonuç<Dize, SağlayıcıHatası> {
        let api_key = ApiKeyManager::gemini().ok_or_ else(|| {
            ProviderError::InvalidInput(
                "GEMINI_API_KEY ortam değişkeni yapılandırılmamış."
                    .to_string(),
            )
        })?;

        Müşteriyi bırak =
            GeminiGenerateClient::new(api_key , self.model.clone());

        istemci.oluştur(istem).bekle
    }

    pub async fn execute_remote(
        &kendisi,
        nesne: &BilimselNesne,
    ) -> Sonuç<BilimselNesne, SağlayıcıHatası> {
        let prompt = object
            .yük
            .get("claim")
            .ve_sonra(|değer| değer.str())
            .unwrap_or(&object.title);

        let response_text = self.generate(prompt).await?;

        let mut output = object.clone();

        if let Some(payload) = output.payload.as_object_mut() {
            yük.ekle(
                "gemini_response".to_string(),
                json!(yanıt_metni),
            );
            yük.ekle(
                "gemini_model".to_string(),
                json!(self.model),
            );
        } başka {
            çıktı.yük = json!({
                "orijinal_yük": object.payload.clone(),
                "gemini_response": yanıt_metni,
                "gemini_model": self.model
            });
        }

        Tamam (çıktı)
    }
}

GeminiProvider için ScientificProvider'ı uygulayın {
    fn id(&self) -> &'static str {
        "ikizler burcu"
    }

    fn name(&self) -> &'static str {
        "Zanistarast Gemini Provider"
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

        metadata.ekle(
            "type".to_string(),
            "cloud-ai-provider".to_string( ) ,
        );
        metadata.ekle(
            "sağlayıcı".to_string(),
            "gemini".to_string(),
        );
        metadata.ekle(
            "model".to_string(),
            kendi modelini klonla(),
        );
        metadata.ekle(
            "deterministic_wrapper".to_string (),
            "true".to_string(),
        );
        metadata.ekle(
            "api_enabled".to_string(),
            ApiKeyManager::gemini().is_some ().to_string(),
        );

        meta veriler
    }
}

GeminiProvider için varsayılan değer uygulanmıştır.
    fn default() -> Self {
        Kendi::yeni()
    }
}




