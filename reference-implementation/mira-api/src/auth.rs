#![allow(dead_code)]

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

        if token.len() < 32 {
            return Err(
                "Müdebbir erişim anahtarı en az 32 karakter olmalıdır."
                    .to_string(),
            );
        }

        Ok(Self {
            expected_token: token,
        })
    }

    /// `Authorization: Bearer <anahtar>` başlığını doğrular.
    pub fn verify_authorization_header(
        &self,
        authorization_header: Option<&str>,
    ) -> bool {
        let Some(header) = authorization_header else {
            return false;
        };

        let Some(provided_token) =
            header.strip_prefix("Bearer ")
        else {
            return false;
        };

        if provided_token.is_empty() {
            return false;
        }

        bool::from(
            self.expected_token
                .as_bytes()
                .ct_eq(provided_token.as_bytes()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOKEN: &str =
        "zanistarast-mudebbir-test-token-0001";

    #[test]
    fn valid_bearer_token_is_accepted() {
        let auth = MudebbirAuth::new(TEST_TOKEN)
            .expect("test token should be accepted");

        assert!(auth.verify_authorization_header(Some(
            "Bearer zanistarast-mudebbir-test-token-0001"
        )));
    }

    #[test]
    fn incorrect_token_is_rejected() {
        let auth = MudebbirAuth::new(TEST_TOKEN)
            .expect("test token should be accepted");

        assert!(!auth.verify_authorization_header(Some(
            "Bearer incorrect-mudebbir-token-000000"
        )));
    }

    #[test]
    fn missing_authorization_header_is_rejected() {
        let auth = MudebbirAuth::new(TEST_TOKEN)
            .expect("test token should be accepted");

        assert!(!auth.verify_authorization_header(None));
    }

    #[test]
    fn non_bearer_authorization_is_rejected() {
        let auth = MudebbirAuth::new(TEST_TOKEN)
            .expect("test token should be accepted");

        assert!(!auth.verify_authorization_header(Some(
            "Basic zanistarast-mudebbir-test-token-0001"
        )));
    }

    #[test]
    fn short_token_configuration_is_rejected() {
        let error = MudebbirAuth::new("too-short")
            .expect_err("short token should fail");

        assert_eq!(
            error,
            "Müdebbir erişim anahtarı en az 32 karakter olmalıdır."
        );
    }

    #[test]
    fn empty_token_configuration_is_rejected() {
        let error = MudebbirAuth::new(" ")
            .expect_err("empty token should fail");

        assert_eq!(
            error,
            "Müdebbir erişim anahtarı boş olamaz."
        );
    }
}
