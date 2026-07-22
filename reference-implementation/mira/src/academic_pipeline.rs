use crate::academic_rules::{evaluate_rules, AcademicRuleReport};
use crate::article_classifier::AcademicArticleType;
use crate::article_templates::template_for;
use crate::publication_priority::{
    calculate_priority, PublicationPriority,
};

#[derive(Debug, Clone)]
pub struct AcademicPipelineResult {
    pub priority: PublicationPriority,
    pub rules: AcademicRuleReport,
}

pub fn run_pipeline(
    article_type: AcademicArticleType,
    has_abstract: bool,
    has_references: bool,
    has_conclusion: bool,
    has_math: bool,
    has_experiments: bool,
) -> AcademicPipelineResult {
    let template = template_for(article_type);

    let rules = evaluate_rules(
        template,
        has_abstract,
        has_references,
        has_conclusion,
    );

    let priority = calculate_priority(
        article_type,
        has_references,
        has_math,
        has_experiments,
    );

    AcademicPipelineResult {
        priority,
        rules,
    }
}


