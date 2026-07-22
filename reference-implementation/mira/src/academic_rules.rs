use crate::article_templates::AcademicTemplate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcademicRuleReport {
    pub passed: bool,
    pub warnings: Vec<String>,
}

pub fn evaluate_rules(
    template: AcademicTemplate,
    has_abstract: bool,
    has_references: bool,
    has_conclusion: bool,
) -> AcademicRuleReport {
    let mut warnings = Vec::new();

    if !has_abstract {
        warnings.push("Missing Abstract".to_string());
    }

    if !has_references {
        warnings.push("Missing References".to_string());
    }

    if !has_conclusion {
        warnings.push("Missing Conclusion".to_string());
    }

    if matches!(template, AcademicTemplate::Mathematical)
        && !has_references
    {
        warnings.push(
            "Mathematical papers should include references".to_string(),
        );
    }

    AcademicRuleReport {
        passed: warnings.is_empty(),
        warnings,
    }
}



