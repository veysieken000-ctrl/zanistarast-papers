#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatexArticle {
    pub title: String,
    pub author: String,
    pub abstract_text: String,
    pub body: String,
    pub bibliography: Option<String>,
}

fn escape_latex(value: &str) -> String {
    value
        .replace('\\', "\\textbackslash{}")
        .replace('&', "\\&")
        .replace('%', "\\%")
        .replace('$', "\\$")
        .replace('#', "\\#")
        .replace('_', "\\_")
        .replace('{', "\\{")
        .replace('}', "\\}")
}
pub fn generate_latex_article(article: &LatexArticle) -> String {
    let title = escape_latex(&article.title);
    let author = escape_latex(&article.author);
    let abstract_text = escape_latex(&article.abstract_text);

    let bibliography = article
        .bibliography
        .as_ref()
        .map(|name| {
            format!(
                "\n\\bibliographystyle{{plain}}\n\\bibliography{{{}}}\n",
                escape_latex(name)
            )
        })
        .unwrap_or_default();

    format!(
        concat!(
            "\\documentclass{{article}}\n",
            "\\usepackage[utf8]{{inputenc}}\n",
            "\\usepackage[T1]{{fontenc}}\n",
            "\n",
            "\\title{{{}}}\n",
            "\\author{{{}}}\n",
            "\n",
            "\\begin{{document}}\n",
            "\\maketitle\n",
            "\n",
            "\\begin{{abstract}}\n",
            "{}\n",
            "\\end{{abstract}}\n",
            "\n",
            "{}\n",
            "{}",
            "\n",
            "\\end{{document}}\n"
        ),
        title,
        author,
        abstract_text,
        article.body,
        bibliography,
    )
}


#[cfg(test)]
mod tests {
    use super::*;
#[test]
fn escapes_special_latex_characters() {
    let article = LatexArticle {
        title: "Rasterast_100%".to_string(),
        author: "Veysi yê MALA SAF & Team".to_string(),
        abstract_text: "Cost is $100 #verified".to_string(),
        body: String::new(),
        bibliography: None,
    };

    let generated = generate_latex_article(&article);

    assert!(generated.contains("\\title{Rasterast\\_100\\%}"));
    assert!(generated.contains("\\author{Veysi yê MALA SAF \\& Team}"));
    assert!(generated.contains("Cost is \\$100 \\#verified"));
}
    
    #[test]
    fn generates_basic_latex_article() {
        let article = LatexArticle {
            title: "Rasterast Verification".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text:
                "This article presents deterministic verification."
                    .to_string(),
            body: "\\section{Introduction}\nZanistarast scientific synthesis."
                .to_string(),
            bibliography: None,
        };

        let generated = generate_latex_article(&article);

        let expected = concat!(
            "\\documentclass{article}\n",
            "\\usepackage[utf8]{inputenc}\n",
            "\\usepackage[T1]{fontenc}\n",
            "\n",
            "\\title{Rasterast Verification}\n",
            "\\author{Veysi yê MALA SAF}\n",
            "\n",
            "\\begin{document}\n",
            "\\maketitle\n",
            "\n",
            "\\begin{abstract}\n",
            "This article presents deterministic verification.\n",
            "\\end{abstract}\n",
            "\n",
            "\\section{Introduction}\n",
            "Zanistarast scientific synthesis.\n",
            "\n",
            "\\end{document}\n"
        );

        assert_eq!(generated, expected);
    }
}

#[test]
fn generates_bibliography_link() {
    let article = LatexArticle {
        title: "Rasterast Verification".to_string(),
        author: "Veysi yê MALA SAF".to_string(),
        abstract_text: "Deterministic verification.".to_string(),
        body: "\\section{Sources}\nSee \\cite{veysi2025}."
            .to_string(),
        bibliography: Some("references".to_string()),
    };

    let generated = generate_latex_article(&article);

    assert!(generated.contains(
        "\\bibliographystyle{plain}\n\\bibliography{references}"
    ));

    assert!(generated.contains(
        "See \\cite{veysi2025}."
    ));
}



