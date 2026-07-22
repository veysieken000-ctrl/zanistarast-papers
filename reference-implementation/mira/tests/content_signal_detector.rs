use zanistarast_mira::article_inventory::ArticleSourceType;
use zanistarast_mira::content_signal_detector::detect_content_signals;

#[test]
fn markdown_detects_all_basic_sections() {
    let content = r#"
# Abstract

Some text.

# Conclusion

Done.

# References

[1] Test

$$
E = mc^2
$$

Experiment benchmark
"#;

    let signals = detect_content_signals(
        &ArticleSourceType::Markdown,
        content,
    );

    assert!(signals.has_abstract);
    assert!(signals.has_references);
    assert!(signals.has_conclusion);
    assert!(signals.has_math);
    assert!(signals.has_experiments);
}

#[test]
fn html_detects_all_basic_sections() {
    let content = r#"
<html>
    <body>
        <section id="abstract">
            <h2>Abstract</h2>
            <p>Some text.</p>
        </section>

        <section id="conclusion">
            <h2>Conclusion</h2>
            <p>Done.</p>
        </section>

        <section id="references">
            <h2>References</h2>
            <p>[1] Test</p>
        </section>

        <math>
            E = mc^2
        </math>

        <p>Experimental benchmark results.</p>
    </body>
</html>
"#;

    let signals = detect_content_signals(
        &ArticleSourceType::Html,
        content,
    );

    assert!(signals.has_abstract);
    assert!(signals.has_references);
    assert!(signals.has_conclusion);
    assert!(signals.has_math);
    assert!(signals.has_experiments);
}

#[test]
fn latex_detects_all_basic_sections() {
    let content = r#"
\documentclass{article}

\begin{document}

\begin{abstract}
Some text.
\end{abstract}

\section{Conclusion}

Done.

\begin{equation}
E = mc^2
\end{equation}

\section{Experiment}

Benchmark results.

\begin{thebibliography}{9}
\bibitem{test}
Test reference.
\end{thebibliography}

\end{document}
"#;

    let signals = detect_content_signals(
        &ArticleSourceType::Latex,
        content,
    );

    assert!(signals.has_abstract);
    assert!(signals.has_references);
    assert!(signals.has_conclusion);
    assert!(signals.has_math);
    assert!(signals.has_experiments);
}

#[test]
fn plain_text_has_no_academic_signals() {
    let content = r#"
Merhaba.

Bu sadece sıradan bir metindir.

Burada özet, kaynakça, matematiksel denklem,
deney veya sonuç bölümü bulunmamaktadır.
"#;

    let signals = detect_content_signals(
        &ArticleSourceType::Markdown,
        content,
    );

    assert!(!signals.has_abstract);
    assert!(!signals.has_references);
    assert!(!signals.has_conclusion);
    assert!(!signals.has_math);
    assert!(!signals.has_experiments);
}

