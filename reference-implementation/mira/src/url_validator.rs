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




