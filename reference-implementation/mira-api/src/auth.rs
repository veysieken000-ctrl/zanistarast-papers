use subtle::ConstantTimeEq;

/// Yalnızca Müdebbirin kullanacağı API erişim anahtarını doğrular.
///
/// Güvenlik kuralları:
/// - anahtar kaynak kodda tutulmaz,
/// - anahtar günlük kayıtlarına yazılmaz,
/// - yalnızca `Bearer <anahtar>` biçimi kabul edilir,
/// - boş veya hatalı başlık reddedilir,
/// - karşılaştırma sabit zamanlı yapılır.
#[derive(Debug, Clone)]
pub struct MudebbirAuth {
    expected_token: String,
}

impl MudebbirAuth {
    /// Ortam değişkeninden Müdebbir erişim anahtarını yükler.
    pub fn from_environment() -> Result<Self, String> {
        let token = std::env::var("MIRA_MUDEBBIR_TOKEN")
            .map_err(|_| {
                "MIRA_MUDEBBIR_TOKEN ortam değişkeni tanımlı değil."
                    .to_string()
            })?;

        Self::new(token)
    }

    /// Testler ve kontrollü kurulum için yeni doğrulayıcı oluşturur.
    pub fn new(token: impl Into<String>) -> Result<Self, String> {
        let token = token.into();

        if token.trim().is_empty() {
            return Err(
                "Müdebbir erişim anahtarı boş olamaz."
                    .to_string(),
            );
        }
