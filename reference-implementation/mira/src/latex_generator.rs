#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatexArticle {
    pub title: String,
    pub author: String,
    pub abstract_text: String,
    pub body: String,
}

pub fn generate_latex_article(article: &LatexArticle) -> String {
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
            "\n",
            "\\end{{document}}\n"
        ),
        article.title,
        article.author,
        article.abstract_text,
        article.body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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





