use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibtexEntry {
    pub entry_type: String,
    pub citation_key: String,
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BibtexParseError {
    MissingEntryPrefix,
    MissingOpeningBrace,
    MissingClosingBrace,
    MissingCitationKey,
    InvalidField,
}

/// Tek bir BibTeX kaydını deterministik olarak ayrıştırır.
///
/// Desteklenen temel biçim:
///
/// `@article{key, title={Example}, year={2025}}`
pub fn parse_bibtex_entry(
    content: &str,
) -> Result<BibtexEntry, BibtexParseError> {
    let trimmed = content.trim();

    let Some(after_at) = trimmed.strip_prefix('@') else {
        return Err(BibtexParseError::MissingEntryPrefix);
    };

    let Some(opening_brace_index) = after_at.find('{') else {
        return Err(BibtexParseError::MissingOpeningBrace);
    };

    if !trimmed.ends_with('}') {
        return Err(BibtexParseError::MissingClosingBrace);
    }

    let entry_type = after_at[..opening_brace_index]
        .trim()
        .to_lowercase();

    let body = &after_at[opening_brace_index + 1..after_at.len() - 1];

    let Some((citation_key, fields_content)) = body.split_once(',') else {
        return Err(BibtexParseError::MissingCitationKey);
    };

    let citation_key = citation_key.trim();

    if citation_key.is_empty() {
        return Err(BibtexParseError::MissingCitationKey);
    }

    let mut fields = BTreeMap::new();

    for raw_field in fields_content.split(',') {
        let raw_field = raw_field.trim();

        if raw_field.is_empty() {
            continue;
        }

        let Some((name, value)) = raw_field.split_once('=') else {
            return Err(BibtexParseError::InvalidField);
        };

        let name = name.trim().to_lowercase();

        let value = value
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .trim_start_matches('"')
            .trim_end_matches('"')
            .trim()
            .to_string();

        if name.is_empty() || value.is_empty() {
            return Err(BibtexParseError::InvalidField);
        }

        fields.insert(name, value);
    }

    Ok(BibtexEntry {
        entry_type,
        citation_key: citation_key.to_string(),
        fields,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_bibtex_entry() {
        let content = r#"
@article{eken2025,
    title={Rasterast Verification},
    author={Veysi Eken},
    year={2025},
    doi={10.1000/rasterast}
}
"#;

        let entry = parse_bibtex_entry(content)
            .expect("BibTeX entry should be parsed");

        assert_eq!(entry.entry_type, "article");
        assert_eq!(entry.citation_key, "eken2025");

        assert_eq!(
            entry.fields.get("title").map(String::as_str),
            Some("Rasterast Verification")
        );

        assert_eq!(
            entry.fields.get("author").map(String::as_str),
            Some("Veysi Eken")
        );

        assert_eq!(
            entry.fields.get("year").map(String::as_str),
            Some("2025")
        );

        assert_eq!(
            entry.fields.get("doi").map(String::as_str),
            Some("10.1000/rasterast")
        );
    }

    #[test]
    fn rejects_invalid_bibtex_entries() {
        assert_eq!(
            parse_bibtex_entry("@article{key, title={Example}}"),
            Err(BibtexParseError::MissingCitationKey)

        );

        assert_eq!(
            parse_bibtex_entry("@article key, title={Example}}"),
            Err(BibtexParseError::MissingOpeningBrace)
        );

        assert_eq!(
            parse_bibtex_entry("@article{key, title={Example}"),
            Err(BibtexParseError::MissingClosingBrace)
        );

        assert_eq!(
            parse_bibtex_entry("@article{, title={Example}}"),
            Err(BibtexParseError::MissingCitationKey)

        );

        assert_eq!(
            parse_bibtex_entry("@article{key, invalid-field}"),
            Err(BibtexParseError::InvalidField)
        );
    }
}



