use crate::article_templates::AcademicTemplate;
use crate::template_sections::required_sections;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateValidationReport {
    pub missing_sections: Vec<String>,
    pub is_complete: bool,
}

pub fn validate_template(
    template: AcademicTemplate,
    existing_sections: &[String],
) -> TemplateValidationReport {
    let required = required_sections(template);

    let mut missing = Vec::new();

    for section in required {
        if !existing_sections.iter().any(|s| s == section) {
            missing.push(section.to_string());
        }
    }

    TemplateValidationReport {
        is_complete: missing.is_empty(),
        missing_sections: missing,
    }
}


