use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationReferenceMatchReport {
    pub citation_numbers: Vec<u32>,
    pub reference_numbers: Vec<u32>,
    pub missing_references: Vec<u32>,
    pub unused_references: Vec<u32>,
}

impl CitationReferenceMatchReport {
    pub fn is_fully_matched(&self) -> bool {
        self.missing_references.is_empty()
            && self.unused_references.is_empty()
    }
}

/// Metindeki `[1]`, `[2]` biçimindeki atıfları ve
/// kaynakça bölümündeki numaralı kayıtları eşleştirir.
///
/// Bu ilk sürüm yalnızca sayısal köşeli parantez biçimini destekler.
pub fn match_citations_and_references(
    content: &str,
) -> CitationReferenceMatchReport {
    let mut citation_numbers = BTreeSet::new();
    let mut reference_numbers = BTreeSet::new();
    let mut inside_references = false;

    for line in content.lines() {
        let trimmed = line.trim();
        let normalized = trimmed.to_lowercase();

        if normalized == "# references"
            || normalized == "## references"
            || normalized == "# kaynakça"
            || normalized == "## kaynakça"
            || normalized == "\\section{references}"
            || normalized == "\\section{kaynakça}"
        {
            inside_references = true;
            continue;
        }

        let numbers = extract_bracketed_numbers(trimmed);

        if inside_references {
            reference_numbers.extend(numbers);
        } else {
            citation_numbers.extend(numbers);
        }
    }

    let missing_references = citation_numbers
        .difference(&reference_numbers)
        .copied()
        .collect();

    let unused_references = reference_numbers
        .difference(&citation_numbers)
        .copied()
        .collect();

    CitationReferenceMatchReport {
        citation_numbers: citation_numbers.into_iter().collect(),
        reference_numbers: reference_numbers.into_iter().collect(),
        missing_references,
        unused_references,
    }
}

fn extract_bracketed_numbers(content: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut remaining = content;

    while let Some(opening_index) = remaining.find('[') {
        let after_opening = &remaining[opening_index + 1..];

        let Some(closing_index) = after_opening.find(']') else {
            break;
        };

        let candidate = after_opening[..closing_index].trim();

        if let Ok(number) = candidate.parse::<u32>() {
            numbers.push(number);
        }

        remaining = &after_opening[closing_index + 1..];
    }

    numbers
}



