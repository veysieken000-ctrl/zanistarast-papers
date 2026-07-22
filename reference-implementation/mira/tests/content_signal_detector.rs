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


