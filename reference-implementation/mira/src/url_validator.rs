/// URL'nin temel HTTP/HTTPS biçimine uyup uymadığını denetler.
///
/// Bu fonksiyon bağlantının gerçekten erişilebilir olduğunu doğrulamaz.
/// Yalnızca deterministik biçim doğrulaması yapar.
pub fn is_valid_url(candidate: &str) -> bool {
    let url = candidate.trim();

    if url.is_empty() || url.chars().any(char::is_whitespace) {
        return false;
    }

    let remainder = if let Some(value) = url.strip_prefix("https://") {
        value
    } else if let Some(value) = url.strip_prefix("http://") {
        value
    } else {
        return false;
    };

    if remainder.is_empty() {
        return false;
    }

    let Some(host) = remainder.split('/').next() else {
        return false;
    };

    if host.is_empty() {
        return false;
    }

    host.contains('.')
        && !host.starts_with('.')
        && !host.ends_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_http_and_https_urls() {
        assert!(is_valid_url("https://example.org"));
        assert!(is_valid_url("http://example.org"));

        assert!(is_valid_url(
            "https://example.org/article"
        ));

        assert!(is_valid_url(
            "https://doi.org/10.1000/xyz123"
        ));

        assert!(is_valid_url(
            "https://sub.example.org/path/file.html"
        ));
    }

    #[test]
    fn rejects_invalid_urls() {
        assert!(!is_valid_url(""));

        assert!(!is_valid_url("example.org"));

        assert!(!is_valid_url("ftp://example.org"));

        assert!(!is_valid_url("https://"));

        assert!(!is_valid_url("https://.example.org"));

       assert!(!is_valid_url("https://example .org"));

        assert!(!is_valid_url(
            "https://example .org"
        ));
    }
}



