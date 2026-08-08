use serde::{Deserialize, Serialize};

/// Zanistarast matematiksel temelindeki
/// biçimsel bilgi öğesi türlerini belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum FormalFoundationKind {
    Axiom,
    Definition,
    Predicate,
    Theorem,
    Lemma,
    Proof,
}

/// Zanistarast matematiksel temelindeki
/// tek bir biçimsel bilgi kaydıdır.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct FormalFoundationRecord {
    pub id: String,
    pub kind: FormalFoundationKind,
    pub statement: String,
}

impl FormalFoundationRecord {
    /// Yeni bir biçimsel temel kaydı oluşturur.
    pub fn new(
        id: impl Into<String>,
        kind: FormalFoundationKind,
        statement: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            statement: statement.into(),
        }
    }

    /// Kaydın zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.statement.trim().is_empty()
    }
}

/// Zanistarast matematiksel temelindeki
/// tek bir biçimsel aksiyomu temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct FormalAxiom {
    pub id: String,
    pub statement: String,
    pub rationale: String,
}

impl FormalAxiom {
    /// Yeni bir biçimsel aksiyom oluşturur.
    pub fn new(
        id: impl Into<String>,
        statement: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            statement: statement.into(),
            rationale: rationale.into(),
        }
    }

    /// Aksiyomun zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.statement.trim().is_empty()
            && !self.rationale.trim().is_empty()
    }

    /// Aksiyomu genel biçimsel temel kaydına dönüştürür.
    pub fn as_foundation_record(
        &self,
    ) -> FormalFoundationRecord {
        FormalFoundationRecord::new(
            self.id.clone(),
            FormalFoundationKind::Axiom,
            self.statement.clone(),
        )
    }
}

