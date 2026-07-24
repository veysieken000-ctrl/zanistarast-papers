#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibtexArticle {
    pub citation_key: String,
    pub author: String,
    pub title: String,
    pub year: u32,
}

pub fn generate_bibtex_article(article: &BibtexArticle) -> String {
    format!(
        "@article{{{},\n author = {{{}}},\n title = {{{}}},\n year = {{{}}}\n}}",
        article.citation_key,
        article.author,
        article.title,
        article.year,
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
}
