#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibtexArticle {
    pub citation_key: String,
    pub author: String,
    pub title: String,
    pub year: u32,
    pub doi: Option<String>,
    pub url: Option<String>,
}

fn escape_bibtex_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('{', "\\{")
        .replace('}', "\\}")
}

pub fn generate_bibtex_article(article: &BibtexArticle) -> String {
    let author = escape_bibtex_value(&article.author);
let title = escape_bibtex_value(&article.title);

let mut fields = vec![
    format!(" author = {{{author}}}"),
    format!(" title = {{{title}}}"),
    format!(" year = {{{}}}", article.year),
];

    if let Some(doi) = &article.doi {
        fields.push(format!(" doi = {{{doi}}}"));
    }

    if let Some(url) = &article.url {
        fields.push(format!(" url = {{{url}}}"));
    }

    format!(
        "@article{{{},\n{}\n}}",
        article.citation_key,
        fields.join(",\n"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_bibtex_article_entry() {
        let article = BibtexArticle {
            citation_key: "veysi2025".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            title: "Rasterast Verification".to_string(),
            year: 2025,
            doi: None,
            url: None,
        };

        let generated = generate_bibtex_article(&article);

        let expected = concat!(
            "@article{veysi2025,\n",
            " author = {Veysi yê MALA SAF},\n",
            " title = {Rasterast Verification},\n",
            " year = {2025}\n",
            "}"
        );

        assert_eq!(generated, expected);
    }

    #[test]
    fn generates_optional_doi_and_url_fields() {
        let article = BibtexArticle {
            citation_key: "veysi2025".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            title: "Zanistarast Scientific Synthesis".to_string(),
            year: 2025,
            doi: Some("10.1000/zanistarast".to_string()),
            url: Some(
                "https://example.org/zanistarast".to_string(),
            ),
        };

        let generated = generate_bibtex_article(&article);

        let expected = concat!(
            "@article{veysi2025,\n",
            " author = {Veysi yê MALA SAF},\n",
            " title = {Zanistarast Scientific Synthesis},\n",
            " year = {2025},\n",
            " doi = {10.1000/zanistarast},\n",
            " url = {https://example.org/zanistarast}\n",
            "}"
        );

        assert_eq!(generated, expected);
    }
}

#[test]
fn escapes_braces_in_bibtex_values() {
    let article = BibtexArticle {
        citation_key: "veysi2025".to_string(),
        author: "Veysi yê {MALA SAF}".to_string(),
        title: "Rasterast {Deterministic} Verification".to_string(),
        year: 2025,
        doi: None,
        url: None,
    };

    let generated = generate_bibtex_article(&article);

    assert!(generated.contains(
        "author = {Veysi yê \\{MALA SAF\\}}"
    ));

    assert!(generated.contains(
        "title = {Rasterast \\{Deterministic\\} Verification}"
    ));
}



