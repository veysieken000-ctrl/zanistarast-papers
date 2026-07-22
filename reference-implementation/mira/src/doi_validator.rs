/// Bir DOI adayını standart ön eklerden arındırır.
fn normalize_doi(candidate: &str) -> &str {
    let trimmed = candidate.trim();
    let lowercase = trimmed.to_ascii_lowercase();

    for prefix in [
        "https://doi.org/",
        "http://doi.org/",
        "doi:",
    ] {
        if lowercase.starts_with(prefix) {
            return trimmed[prefix.len()..].trim();
        }
    }

    trimmed
}

/// DOI metninin temel biçim kurallarına uyup uymadığını denetler.
///
/// Kabul edilen örnekler:
/// - `10.1000/xyz123`
/// - `doi:10.1000/xyz123`
/// - `https://doi.org/10.1000/xyz123`
///
/// Bu fonksiyon DOI kaydının gerçekten mevcut olduğunu doğrulamaz.
/// Yalnızca yerel ve deterministik biçim doğrulaması yapar.
pub fn is_valid_doi(candidate: &str) -> bool {
    let doi = normalize_doi(candidate);

    if doi.is_empty() || doi.chars().any(char::is_whitespace) {
        return false;
    }

    let Some((prefix, suffix)) = doi.split_once('/') else {
        return false;
    };

    if suffix.is_empty() {
        return false;
    }

    let Some(registrant) = prefix.strip_prefix("10.") else {
        return false;
    };

    if !(4..=9).contains(&registrant.len()) {
        return false;
    }

    if !registrant.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }

    suffix.chars().all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(
                character,
                '-' | '.'
                    | '_'
                    | ';'
                    | '('
                    | ')'
                    | '/'
                    | ':'
            )
    })
}



