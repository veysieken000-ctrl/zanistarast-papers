use crate::article_candidate_analysis::ArticleMaturityLevel;
use crate::article_inventory::ZanistarastDomain;
use crate::topic_clustering::{
    ClusteredArticle,
    TopicClusteringReport,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Bilgi haritasındaki tek bir bilimsel içerik düğümü.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeNode {
    pub id: String,
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub readiness_score: u8,
    pub maturity_level: ArticleMaturityLevel,
}

/// Aynı bilgi alanındaki iki düğüm arasındaki ilişki.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: KnowledgeRelationType,
}

/// Bilgi haritasında kullanılabilecek ilişki türleri.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KnowledgeRelationType {
    SameDomain,
    PossibleContinuation,
    RequiresReview,
}

/// Bir bilgi düğümünün Zanistarast bilgi mimarisindeki
/// işlevsel katmanını belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastKnowledgeLayer {
    /// Değişmez çekirdek ilkeler, kavramlar,
    /// aksiyomlar, epistemik hükümler ve resmî kararlar.
    Dna,

    /// Görevler, süreçler, dönüşüm kuralları
    /// ve bilgi aktarım mekanizmaları.
    Rna,

    /// Makaleler, kod modülleri, raporlar,
    /// yayın paketleri ve diğer somut çıktılar.
    Protein,
}

/// Zanistarast DNA katmanındaki değişmez veya
/// otoriteye bağlı çekirdek bilgi türlerini belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastDnaKind {
    /// Zanistarast’ın değişmez temel ilkesi.
    CorePrinciple,

    /// Tanımlanmış temel veya türetilmiş kavram.
    Concept,

    /// Biçimsel veya kurucu aksiyom.
    Axiom,

    /// Bilginin doğruluk ve kesinlik statüsüne
    /// ilişkin epistemik hüküm.
    EpistemicJudgment,

    /// Müdebbir tarafından verilmiş resmî karar.
    OfficialDecision,
}

/// Bir bilgi düğümünün Zanistarast DNA katmanındaki
/// ayrıntılı ve gerekçeli kaydıdır.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct DnaKnowledgeRecord {
    pub node_id: String,
    pub kind: ZanistarastDnaKind,
    pub rationale: String,
    pub immutable: bool,
}

impl DnaKnowledgeRecord {
    /// Yeni bir DNA bilgi kaydı oluşturur.
    pub fn new(
        node_id: impl Into<String>,
        kind: ZanistarastDnaKind,
        rationale: impl Into<String>,
        immutable: bool,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind,
            rationale: rationale.into(),
            immutable,
        }
    }

    /// DNA kaydının zorunlu bilgilerinin eksiksiz
    /// ve DNA ilkeleriyle uyumlu olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
            && self.immutable
    }

    /// DNA kaydının belirtilen türde olup
    /// olmadığını bildirir.
    pub fn is_kind(
        &self,
        kind: ZanistarastDnaKind,
    ) -> bool {
        self.kind == kind
    }
}

/// Zanistarast DNA katmanındaki ayrıntılı
/// bilgi kayıtlarının koleksiyonudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct DnaKnowledgeMap {
    pub records: Vec<DnaKnowledgeRecord>,
}

impl DnaKnowledgeMap {
    /// Boş bir DNA bilgi haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve aynı düğüm için daha önce
    /// kaydedilmemiş bir DNA kaydı ekler.
    pub fn register(
        &mut self,
        record: DnaKnowledgeRecord,
    ) -> bool {
        if !record.is_complete() {
            return false;
        }

        if self.records.iter().any(|stored| {
            stored.node_id == record.node_id
        }) {
            return false;
        }

        self.records.push(record);
        true
    }

    /// Toplam DNA bilgi kaydı sayısını döndürür.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// DNA bilgi haritasının boş olup
    /// olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Belirtilen düğüme ait DNA kaydını döndürür.
    pub fn record_for_node(
        &self,
        node_id: &str,
    ) -> Option<&DnaKnowledgeRecord> {
        self.records.iter().find(|record| {
            record.node_id == node_id
        })
    }

    /// Belirtilen DNA türüne ait bütün
    /// kayıtları döndürür.
    pub fn records_of_kind(
        &self,
        kind: ZanistarastDnaKind,
    ) -> Vec<&DnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_kind(kind))
            .collect()
    }

    /// Bir düğümün DNA katmanında kayıtlı olup
    /// olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.record_for_node(node_id).is_some()
    }
}

/// Zanistarast RNA katmanındaki görev, süreç ve
/// bilgi dönüşümü türlerini belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastRnaKind {
    /// Mira veya başka bir ajan tarafından
    /// yürütülecek açık görev.
    Task,

    /// Birden fazla adımı içeren iş veya
    /// akademik üretim süreci.
    Process,

    /// Bir bilgi biçimini başka bir bilgi veya
    /// çıktı biçimine dönüştüren kural.
    TransformationRule,

    /// Bilginin katmanlar, ajanlar veya sistemler
    /// arasında taşınmasını sağlayan mekanizma.
    KnowledgeTransfer,

    /// Rasterast tarafından yürütülecek
    /// doğrulama süreci.
    VerificationRequest,

    /// Müdebbir kararı gerektiren onay süreci.
    ApprovalRequest,
}

/// Bir bilgi düğümünün Zanistarast RNA katmanındaki
/// ayrıntılı ve gerekçeli süreç kaydıdır.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RnaKnowledgeRecord {
    pub node_id: String,
    pub kind: ZanistarastRnaKind,
    pub rationale: String,
    pub source_node_ids: Vec<String>,
    pub target_node_ids: Vec<String>,
}

impl RnaKnowledgeRecord {
    /// Yeni bir RNA bilgi kaydı oluşturur.
    pub fn new(
        node_id: impl Into<String>,
        kind: ZanistarastRnaKind,
        rationale: impl Into<String>,
        source_node_ids: Vec<String>,
        target_node_ids: Vec<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind,
            rationale: rationale.into(),
            source_node_ids,
            target_node_ids,
        }
    }

    /// RNA kaydının zorunlu bilgilerinin eksiksiz
    /// olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
            && self
                .source_node_ids
                .iter()
                .all(|node_id| !node_id.trim().is_empty())
            && self
                .target_node_ids
                .iter()
                .all(|node_id| !node_id.trim().is_empty())
    }

    /// RNA kaydının belirtilen türde olup
    /// olmadığını bildirir.
    pub fn is_kind(
        &self,
        kind: ZanistarastRnaKind,
    ) -> bool {
        self.kind == kind
    }

    /// RNA sürecinin belirtilen bilgi düğümünden
    /// beslenip beslenmediğini bildirir.
    pub fn uses_source_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.source_node_ids
            .iter()
            .any(|source| source == node_id)
    }

    /// RNA sürecinin belirtilen bilgi düğümünü
    /// üretip üretmediğini bildirir.
    pub fn produces_target_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.target_node_ids
            .iter()
            .any(|target| target == node_id)
    }
}

/// Zanistarast RNA katmanındaki görev, süreç ve
/// bilgi dönüşümü kayıtlarının koleksiyonudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RnaKnowledgeMap {
    pub records: Vec<RnaKnowledgeRecord>,
}

impl RnaKnowledgeMap {
    /// Boş bir RNA bilgi haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve aynı düğüm için daha önce
    /// kaydedilmemiş bir RNA kaydı ekler.
    pub fn register(
        &mut self,
        record: RnaKnowledgeRecord,
    ) -> bool {
        if !record.is_complete() {
            return false;
        }

        if self.records.iter().any(|stored| {
            stored.node_id == record.node_id
        }) {
            return false;
        }

        self.records.push(record);
        true
    }

    /// Toplam RNA bilgi kaydı sayısını döndürür.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// RNA bilgi haritasının boş olup
    /// olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Belirtilen düğüme ait RNA kaydını döndürür.
    pub fn record_for_node(
        &self,
        node_id: &str,
    ) -> Option<&RnaKnowledgeRecord> {
        self.records.iter().find(|record| {
            record.node_id == node_id
        })
    }

    /// Belirtilen RNA türüne ait bütün
    /// kayıtları döndürür.
    pub fn records_of_kind(
        &self,
        kind: ZanistarastRnaKind,
    ) -> Vec<&RnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_kind(kind))
            .collect()
    }

    /// Belirtilen bilgi düğümünü kaynak olarak
    /// kullanan RNA kayıtlarını döndürür.
    pub fn records_using_source(
        &self,
        source_node_id: &str,
    ) -> Vec<&RnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| {
                record.uses_source_node(source_node_id)
            })
            .collect()
    }

    /// Belirtilen bilgi düğümünü çıktı olarak
    /// üreten RNA kayıtlarını döndürür.
    pub fn records_producing_target(
        &self,
        target_node_id: &str,
    ) -> Vec<&RnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| {
                record.produces_target_node(target_node_id)
            })
            .collect()
    }

    /// Bir düğümün RNA katmanında kayıtlı olup
    /// olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.record_for_node(node_id).is_some()
    }

    }

    /// Zanistarast Protein katmanındaki somut
/// bilimsel ve teknik çıktı türlerini belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastProteinKind {
    /// Akademik makale veya makale taslağı.
    Article,

    /// Çalışan veya hazırlanmakta olan kod modülü.
    CodeModule,

    /// Bilimsel, teknik veya doğrulama raporu.
    Report,

    /// Yayına hazır dosya ve metadata paketi.
    PublicationPackage,

    /// Website, servis veya başka bir uygulama çıktısı.
    Application,
}

/// Zanistarast Protein katmanındaki somut
/// bilimsel ve teknik çıktı türlerini belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastProteinKind {
    /// Akademik makale veya makale taslağı.
    Article,

    /// Çalışan veya hazırlanmakta olan kod modülü.
    CodeModule,

    /// Bilimsel, teknik veya doğrulama raporu.
    Report,

    /// Yayına hazır dosya ve metadata paketi.
    PublicationPackage,

    /// Website, servis veya başka bir uygulama çıktısı.
    Application,
}

/// Zanistarast Protein katmanındaki somut bir
/// çıktının ayrıntılı ve gerekçeli kaydıdır.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct ProteinKnowledgeRecord {
    pub node_id: String,
    pub kind: ZanistarastProteinKind,
    pub rationale: String,
    pub source_node_ids: Vec<String>,
    pub relative_path: Option<PathBuf>,
    pub verified: bool,
}

impl ProteinKnowledgeRecord {
    /// Yeni bir Protein bilgi kaydı oluşturur.
    pub fn new(
        node_id: impl Into<String>,
        kind: ZanistarastProteinKind,
        rationale: impl Into<String>,
        source_node_ids: Vec<String>,
        relative_path: Option<PathBuf>,
        verified: bool,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind,
            rationale: rationale.into(),
            source_node_ids,
            relative_path,
            verified,
        }
    }

    /// Protein kaydının zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
            && self
                .source_node_ids
                .iter()
                .all(|node_id| !node_id.trim().is_empty())
            && self
                .relative_path
                .as_ref()
                .is_none_or(|path| {
                    !path.as_os_str().is_empty()
                })
    }

    /// Protein kaydının belirtilen türde olup
    /// olmadığını bildirir.
    pub fn is_kind(
        &self,
        kind: ZanistarastProteinKind,
    ) -> bool {
        self.kind == kind
    }

    /// Protein çıktısının belirtilen bilgi
    /// düğümünden üretilip üretilmediğini bildirir.
    pub fn uses_source_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.source_node_ids
            .iter()
            .any(|source| source == node_id)
    }

    /// Protein çıktısının dosya veya dizin yoluna
    /// bağlı olup olmadığını bildirir.
    pub fn has_relative_path(&self) -> bool {
        self.relative_path
            .as_ref()
            .is_some_and(|path| {
                !path.as_os_str().is_empty()
            })
    }

    /// Protein çıktısının doğrulanmış olup
    /// olmadığını bildirir.
    pub fn is_verified(&self) -> bool {
        self.verified
    }
}

/// Zanistarast Protein katmanındaki somut
/// çıktı kayıtlarının koleksiyonudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct ProteinKnowledgeMap {
    pub records: Vec<ProteinKnowledgeRecord>,
}

impl ProteinKnowledgeMap {
    /// Boş bir Protein bilgi haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve aynı düğüm için daha önce
    /// kaydedilmemiş bir Protein kaydı ekler.
    pub fn register(
        &mut self,
        record: ProteinKnowledgeRecord,
    ) -> bool {
        if !record.is_complete() {
            return false;
        }

        if self.records.iter().any(|stored| {
            stored.node_id == record.node_id
        }) {
            return false;
        }

        self.records.push(record);
        true
    }

    /// Toplam Protein bilgi kaydı sayısını döndürür.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Protein bilgi haritasının boş olup
    /// olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Belirtilen düğüme ait Protein kaydını döndürür.
    pub fn record_for_node(
        &self,
        node_id: &str,
    ) -> Option<&ProteinKnowledgeRecord> {
        self.records.iter().find(|record| {
            record.node_id == node_id
        })
    }

    /// Belirtilen Protein türüne ait bütün
    /// kayıtları döndürür.
    pub fn records_of_kind(
        &self,
        kind: ZanistarastProteinKind,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_kind(kind))
            .collect()
    }

    /// Belirtilen bilgi düğümünden üretilmiş
    /// Protein kayıtlarını döndürür.
    pub fn records_using_source(
        &self,
        source_node_id: &str,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| {
                record.uses_source_node(source_node_id)
            })
            .collect()
    }

    /// Doğrulanmış Protein çıktılarını döndürür.
    pub fn verified_records(
        &self,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_verified())
            .collect()
    }

    /// Henüz doğrulanmamış Protein çıktılarını döndürür.
    pub fn unverified_records(
        &self,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| !record.is_verified())
            .collect()
    }

    /// Bir düğümün Protein katmanında kayıtlı olup
    /// olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.record_for_node(node_id).is_some()
    }
}

/// Zanistarast Protein katmanındaki somut
/// çıktı kayıtlarının koleksiyonudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct ProteinKnowledgeMap {
    pub records: Vec<ProteinKnowledgeRecord>,
}

impl ProteinKnowledgeMap {
    /// Boş bir Protein bilgi haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve aynı düğüm için daha önce
    /// kaydedilmemiş bir Protein kaydı ekler.
    pub fn register(
        &mut self,
        record: ProteinKnowledgeRecord,
    ) -> bool {
        if !record.is_complete() {
            return false;
        }

        if self.records.iter().any(|stored| {
            stored.node_id == record.node_id
        }) {
            return false;
        }

        self.records.push(record);
        true
    }

    /// Toplam Protein bilgi kaydı sayısını döndürür.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Protein bilgi haritasının boş olup
    /// olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Belirtilen düğüme ait Protein kaydını döndürür.
    pub fn record_for_node(
        &self,
        node_id: &str,
    ) -> Option<&ProteinKnowledgeRecord> {
        self.records.iter().find(|record| {
            record.node_id == node_id
        })
    }

    /// Belirtilen Protein türüne ait bütün
    /// kayıtları döndürür.
    pub fn records_of_kind(
        &self,
        kind: ZanistarastProteinKind,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_kind(kind))
            .collect()
    }

    /// Belirtilen bilgi düğümünden üretilmiş
    /// Protein kayıtlarını döndürür.
    pub fn records_using_source(
        &self,
        source_node_id: &str,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| {
                record.uses_source_node(source_node_id)
            })
            .collect()
    }

    /// Doğrulanmış Protein çıktılarını döndürür.
    pub fn verified_records(
        &self,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_verified())
            .collect()
    }

    /// Henüz doğrulanmamış Protein çıktılarını döndürür.
    pub fn unverified_records(
        &self,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| !record.is_verified())
            .collect()
    }

    /// Bir düğümün Protein katmanında kayıtlı olup
    /// olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.record_for_node(node_id).is_some()
    }
}

/// Mevcut bir bilgi düğümünün DNA–RNA–Protein
/// mimarisindeki katman atamasını temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]

pub struct KnowledgeLayerAssignment {
    pub node_id: String,
    pub layer: ZanistarastKnowledgeLayer,
    pub rationale: String,
}

impl KnowledgeLayerAssignment {
    /// Yeni ve gerekçeli bir katman ataması oluşturur.
    pub fn new(
        node_id: impl Into<String>,
        layer: ZanistarastKnowledgeLayer,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            layer,
            rationale: rationale.into(),
        }
    }

    /// Katman atamasının zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
    }
}
/// Zanistarast DNA–RNA–Protein bilgi mimarisinin
/// birleşik ve sorgulanabilir modelidir.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct ZanistarastKnowledgeArchitecture {
    pub dna: DnaKnowledgeMap,
    pub rna: RnaKnowledgeMap,
    pub protein: ProteinKnowledgeMap,
}

impl ZanistarastKnowledgeArchitecture {
    /// Boş bir Zanistarast bilgi mimarisi oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bütün katmanlardaki toplam ayrıntılı
    /// bilgi kaydı sayısını döndürür.
    pub fn total_record_count(&self) -> usize {
        self.dna.record_count()
            + self.rna.record_count()
            + self.protein.record_count()
    }

    /// Bütün bilgi katmanlarının boş olup
    /// olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.dna.is_empty()
            && self.rna.is_empty()
            && self.protein.is_empty()
    }

    /// Belirtilen düğümün herhangi bir ayrıntılı
    /// bilgi katmanında kayıtlı olup olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.dna.contains_node(node_id)
            || self.rna.contains_node(node_id)
            || self.protein.contains_node(node_id)
    }

    /// Belirtilen düğümün ayrıntılı bilgi
    /// mimarisindeki katmanını döndürür.
    pub fn layer_for_node(
        &self,
        node_id: &str,
    ) -> Option<ZanistarastKnowledgeLayer> {
        if self.dna.contains_node(node_id) {
            return Some(ZanistarastKnowledgeLayer::Dna);
        }

        if self.rna.contains_node(node_id) {
            return Some(ZanistarastKnowledgeLayer::Rna);
        }

        if self.protein.contains_node(node_id) {
            return Some(ZanistarastKnowledgeLayer::Protein);
        }

        None
    }

    /// DNA–RNA–Protein ayrıntılı kayıtlarının genel
    /// katman atamalarıyla uyumluluğunu doğrular.
    pub fn validate_layer_alignment(
        &self,
        layer_map: &KnowledgeLayerMap,
    ) -> KnowledgeArchitectureValidationReport {
        let mut missing_detailed_records = Vec::new();
        let mut mismatched_layer_nodes = Vec::new();

        for assignment in &layer_map.assignments {
            match self.layer_for_node(
                &assignment.node_id,
            ) {
                None => {
                    missing_detailed_records.push(
                        assignment.node_id.clone(),
                    );
                }

                Some(actual_layer)
                    if actual_layer
                        != assignment.layer =>
                {
                    mismatched_layer_nodes.push(
                        assignment.node_id.clone(),
                    );
                }

                Some(_) => {}
            }
        }

        let mut unassigned_detailed_nodes =
            Vec::new();

        for record in &self.dna.records {
            if layer_map
                .assignment_for_node(&record.node_id)
                .is_none()
            {
                unassigned_detailed_nodes.push(
                    record.node_id.clone(),
                );
            }
        }

        for record in &self.rna.records {
            if layer_map
                .assignment_for_node(&record.node_id)
                .is_none()
            {
                unassigned_detailed_nodes.push(
                    record.node_id.clone(),
                );
            }
        }

        for record in &self.protein.records {
            if layer_map
                .assignment_for_node(&record.node_id)
                .is_none()
            {
                unassigned_detailed_nodes.push(
                    record.node_id.clone(),
                );
            }
        }

        missing_detailed_records.sort();
        missing_detailed_records.dedup();

        mismatched_layer_nodes.sort();
        mismatched_layer_nodes.dedup();

        unassigned_detailed_nodes.sort();
        unassigned_detailed_nodes.dedup();

        KnowledgeArchitectureValidationReport {
            assignment_count:
                layer_map.assignment_count(),
            detailed_record_count:
                self.total_record_count(),
            missing_detailed_records,
            mismatched_layer_nodes,
            unassigned_detailed_nodes,
        }
    }
}

/// Zanistarast DNA–RNA–Protein ayrıntılı bilgi
/// kayıtlarının katman atamalarıyla uyumluluk raporudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeArchitectureValidationReport {
    pub assignment_count: usize,
    pub detailed_record_count: usize,
    pub missing_detailed_records: Vec<String>,
    pub mismatched_layer_nodes: Vec<String>,
    pub unassigned_detailed_nodes: Vec<String>,
}

impl KnowledgeArchitectureValidationReport {
    /// Ayrıntılı kayıtlar ile katman atamalarının
    /// tamamen uyumlu olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.missing_detailed_records.is_empty()
            && self.mismatched_layer_nodes.is_empty()
            && self.unassigned_detailed_nodes.is_empty()
    }

    /// Katman ataması bulunduğu hâlde ayrıntılı
    /// DNA, RNA veya Protein kaydı bulunmayan
    /// düğüm sayısını döndürür.
    pub fn missing_detailed_record_count(
        &self,
    ) -> usize {
        self.missing_detailed_records.len()
    }

    /// Atanan katman ile ayrıntılı kaydın gerçek
    /// katmanı uyuşmayan düğüm sayısını döndürür.
    pub fn mismatched_layer_count(&self) -> usize {
        self.mismatched_layer_nodes.len()
    }

    /// Ayrıntılı kaydı bulunduğu hâlde genel katman
    /// ataması yapılmamış düğüm sayısını döndürür.
    pub fn unassigned_detailed_node_count(
        &self,
    ) -> usize {
        self.unassigned_detailed_nodes.len()
    }
}

/// Mevcut bilgi haritası düğümlerinin Zanistarast
/// DNA–RNA–Protein katman atamalarını taşır.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeLayerMap {
    pub assignments: Vec<KnowledgeLayerAssignment>,
}

impl KnowledgeLayerMap {
    /// Boş bir katman haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve daha önce aynı düğüm için
    /// kaydedilmemiş bir katman ataması ekler.
    pub fn assign(
        &mut self,
        assignment: KnowledgeLayerAssignment,
    ) -> bool {
        if !assignment.is_complete() {
            return false;
        }

        if self.assignments.iter().any(|stored| {
            stored.node_id == assignment.node_id
        }) {
            return false;
        }

        self.assignments.push(assignment);
        true
    }

    /// Belirtilen düğümün katman atamasını döndürür.
    pub fn assignment_for_node(
        &self,
        node_id: &str,
    ) -> Option<&KnowledgeLayerAssignment> {
        self.assignments
            .iter()
            .find(|assignment| {
                assignment.node_id == node_id
            })
    }

    /// Belirtilen katmana atanmış bütün düğüm
    /// kayıtlarını döndürür.
    pub fn assignments_for_layer(
        &self,
        layer: ZanistarastKnowledgeLayer,
    ) -> Vec<&KnowledgeLayerAssignment> {
        self.assignments
            .iter()
            .filter(|assignment| {
                assignment.layer == layer
            })
            .collect()
    }

    /// Toplam katman ataması sayısını döndürür.
    pub fn assignment_count(&self) -> usize {
        self.assignments.len()
    }

    /// Katman haritasının boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
  /// Katman haritasındaki bütün atamaların mevcut
    /// bilgi haritası düğümlerine ait olup olmadığını
    /// doğrular.
    /// Katman atamaları ile mevcut bilgi haritası
/// düğümlerinin iki yönlü uyumluluğunu doğrular.
///
/// Bilgi haritasında bulunmayan atamalar ile henüz
/// katman atanmamış bilgi düğümleri ayrı raporlanır.
pub fn validate_against(
    &self,
    knowledge_map: &KnowledgeMapReport,
) -> KnowledgeLayerValidationReport {
    let mut knowledge_node_ids = knowledge_map
        .maps
        .iter()
        .flat_map(|map| map.nodes.iter())
        .map(|node| node.id.clone())
        .collect::<Vec<_>>();

    knowledge_node_ids.sort();
    knowledge_node_ids.dedup();

    let mut assigned_node_ids = self
        .assignments
        .iter()
        .map(|assignment| assignment.node_id.clone())
        .collect::<Vec<_>>();

    assigned_node_ids.sort();
    assigned_node_ids.dedup();

    let mut unknown_node_ids = assigned_node_ids
        .iter()
        .filter(|node_id| {
            !knowledge_node_ids.contains(node_id)
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut unassigned_node_ids = knowledge_node_ids
        .iter()
        .filter(|node_id| {
            !assigned_node_ids.contains(node_id)
        })
        .cloned()
        .collect::<Vec<_>>();

    unknown_node_ids.sort();
    unknown_node_ids.dedup();

    unassigned_node_ids.sort();
    unassigned_node_ids.dedup();

    KnowledgeLayerValidationReport {
        assignment_count: self.assignments.len(),
        knowledge_node_count:
            knowledge_node_ids.len(),
        unknown_node_ids,
        unassigned_node_ids,
    }
}
}

       /// DNA–RNA–Protein katman atamalarının mevcut
/// bilgi haritasıyla iki yönlü uyumluluk sonucudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeLayerValidationReport {
    pub assignment_count: usize,
    pub knowledge_node_count: usize,
    pub unknown_node_ids: Vec<String>,
    pub unassigned_node_ids: Vec<String>,
}

impl KnowledgeLayerValidationReport {
    /// Bütün katman atamalarının mevcut düğümlere
    /// bağlı ve bütün bilgi düğümlerinin katman
    /// atanmış olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.unknown_node_ids.is_empty()
            && self.unassigned_node_ids.is_empty()
    }

    /// Bilgi haritasında bulunmayan düğüm kimliği
    /// sayısını döndürür.
    pub fn unknown_node_count(&self) -> usize {
        self.unknown_node_ids.len()
    }

    /// Henüz DNA, RNA veya Protein katmanına
    /// atanmamış düğüm sayısını döndürür.
    pub fn unassigned_node_count(&self) -> usize {
        self.unassigned_node_ids.len()
    }

    /// Katman ataması gerektiren herhangi bir bilgi
    /// düğümü bulunup bulunmadığını bildirir.
    pub fn has_unassigned_nodes(&self) -> bool {
        !self.unassigned_node_ids.is_empty()
    }
} 

/// Tek bir Zanistarast alanına ait bilgi haritası.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DomainKnowledgeMap {
    pub domain: ZanistarastDomain,
    pub nodes: Vec<KnowledgeNode>,
    pub relations: Vec<KnowledgeRelation>,
}

impl DomainKnowledgeMap {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }
}

/// Tüm Zanistarast bilgi haritalarının birleşik raporu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeMapReport {
    pub maps: Vec<DomainKnowledgeMap>,
}

impl KnowledgeMapReport {
    pub fn map_count(&self) -> usize {
        self.maps.len()
    }

    pub fn map_for_domain(
        &self,
        domain: &ZanistarastDomain,
    ) -> Option<&DomainKnowledgeMap> {
        self.maps
            .iter()
            .find(|map| &map.domain == domain)
    }
}

/// Konu kümelerinden salt okunur bilgi haritaları üretir.
///
/// Bu işlem:
/// - orijinal metinleri değiştirmez,
/// - dosya taşımaz,
/// - dosya silmez,
/// - yalnızca mevcut kümeleri düğüm ve ilişkilere dönüştürür.
#[derive(Debug, Default)]
pub struct KnowledgeMapBuilder;

impl KnowledgeMapBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(
        &self,
        clustering_report: &TopicClusteringReport,
    ) -> KnowledgeMapReport {
        let mut maps = clustering_report
            .clusters
            .iter()
            .filter(|cluster| {
                cluster.domain != ZanistarastDomain::Unclassified
            })
            .map(|cluster| {
                let nodes = cluster
                    .articles
                    .iter()
                    .map(Self::node_from_article)
                    .collect::<Vec<_>>();

                let relations = Self::build_relations(&nodes);

                DomainKnowledgeMap {
                    domain: cluster.domain.clone(),
                    nodes,
                    relations,
                }
            })
            .collect::<Vec<_>>();

        maps.sort_by(|left, right| left.domain.cmp(&right.domain));

        KnowledgeMapReport { maps }
    }

    fn node_from_article(
        article: &ClusteredArticle,
    ) -> KnowledgeNode {
        KnowledgeNode {
            id: Self::node_id(&article.relative_path),
            relative_path: article.relative_path.clone(),
            title: article.title.clone(),
            readiness_score: article.readiness_score,
            maturity_level: article.maturity_level.clone(),
        }
    }

    fn node_id(path: &std::path::Path) -> String {
        path.to_string_lossy()
            .chars()
            .map(|character| {
                if character.is_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn build_relations(
        nodes: &[KnowledgeNode],
    ) -> Vec<KnowledgeRelation> {
        let mut relations = Vec::new();

        for (index, source) in nodes.iter().enumerate() {
            for target in nodes.iter().skip(index + 1) {
                relations.push(KnowledgeRelation {
                    source_id: source.id.clone(),
                    target_id: target.id.clone(),
                    relation_type: Self::relation_type(
                        source,
                        target,
                    ),
                });
            }
        }

        relations
    }

    fn relation_type(
        source: &KnowledgeNode,
        target: &KnowledgeNode,
    ) -> KnowledgeRelationType {
        if source.maturity_level == ArticleMaturityLevel::Fragment
            || target.maturity_level
                == ArticleMaturityLevel::Fragment
        {
            return KnowledgeRelationType::RequiresReview;
        }

        let score_difference = source
            .readiness_score
            .abs_diff(target.readiness_score);

        if score_difference <= 15 {
            KnowledgeRelationType::PossibleContinuation
        } else {
            KnowledgeRelationType::SameDomain
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topic_clustering::{
        TopicCluster,
        TopicClusteringReport,
    };

    #[test]
    fn builder_creates_domain_knowledge_maps() {
        let clustering_report = TopicClusteringReport {
            clusters: vec![
                TopicCluster {
                    domain: ZanistarastDomain::Hebun,
                    articles: vec![
                        ClusteredArticle {
                            relative_path: PathBuf::from(
                                "papers/hebun-main.md",
                            ),
                            title: Some(
                                "Hebûn Ana Makalesi".to_string(),
                            ),
                            readiness_score: 90,
                            maturity_level:
                                ArticleMaturityLevel::StrongCandidate,
                        },
                        ClusteredArticle {
                            relative_path: PathBuf::from(
                                "papers/hebun-method.md",
                            ),
                            title: Some(
                                "Hebûn Yöntemi".to_string(),
                            ),
                            readiness_score: 80,
                            maturity_level:
                                ArticleMaturityLevel::DevelopingDraft,
                        },
                    ],
                    total_readiness_score: 170,
                },
                TopicCluster {
                    domain: ZanistarastDomain::Rabun,
                    articles: vec![
                        ClusteredArticle {
                            relative_path: PathBuf::from(
                                "papers/rabun.md",
                            ),
                            title: Some(
                                "Rabûn Yönetim Modeli".to_string(),
                            ),
                            readiness_score: 75,
                            maturity_level:
                                ArticleMaturityLevel::DevelopingDraft,
                        },
                    ],
                    total_readiness_score: 75,
                },
            ],
            total_clustered_articles: 3,
        };

        let builder = KnowledgeMapBuilder::new();
        let report = builder.build(&clustering_report);

        assert_eq!(report.map_count(), 2);

        let hebun_map = report
            .map_for_domain(&ZanistarastDomain::Hebun)
            .expect("Hebûn knowledge map should exist");

        assert_eq!(hebun_map.node_count(), 2);
        assert_eq!(hebun_map.relation_count(), 1);
        assert_eq!(
            hebun_map.relations[0].relation_type,
            KnowledgeRelationType::PossibleContinuation
        );
    }

    #[test]
    fn unclassified_cluster_is_not_mapped() {
        let clustering_report = TopicClusteringReport {
            clusters: vec![TopicCluster {
                domain: ZanistarastDomain::Unclassified,
                articles: vec![ClusteredArticle {
                    relative_path: PathBuf::from(
                        "notes/general.md",
                    ),
                    title: None,
                    readiness_score: 10,
                    maturity_level:
                        ArticleMaturityLevel::Fragment,
                }],
                total_readiness_score: 10,
            }],
            total_clustered_articles: 1,
        };

        let builder = KnowledgeMapBuilder::new();
        let report = builder.build(&clustering_report);

        assert_eq!(report.map_count(), 0);
    }

    #[test]
    fn fragment_relation_requires_review() {
        let nodes = vec![
            KnowledgeNode {
                id: "hebun-main".to_string(),
                relative_path: PathBuf::from(
                    "papers/hebun-main.md",
                ),
                title: Some("Hebûn".to_string()),
                readiness_score: 90,
                maturity_level:
                    ArticleMaturityLevel::StrongCandidate,
            },
            KnowledgeNode {
                id: "hebun-note".to_string(),
                relative_path: PathBuf::from(
                    "notes/hebun-note.md",
                ),
                title: None,
                readiness_score: 15,
                maturity_level:
                    ArticleMaturityLevel::Fragment,
            },
        ];

        let relations =
            KnowledgeMapBuilder::build_relations(&nodes);

        assert_eq!(relations.len(), 1);
        assert_eq!(
            relations[0].relation_type,
            KnowledgeRelationType::RequiresReview
        );
    }

    #[test]
fn assigns_knowledge_nodes_to_dna_rna_protein_layers() {
    let mut layer_map = KnowledgeLayerMap::new();

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-core-principle",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn, Zanistarast çekirdek kavramıdır.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "article-production-process",
            ZanistarastKnowledgeLayer::Rna,
            "Makale üretim görevlerini çıktıya dönüştürür.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "rasterast-paper",
            ZanistarastKnowledgeLayer::Protein,
            "Akademik makale somut bir üretim çıktısıdır.",
        ),
    ));

    assert_eq!(layer_map.assignment_count(), 3);

    assert_eq!(
        layer_map
            .assignments_for_layer(
                ZanistarastKnowledgeLayer::Dna,
            )
            .len(),
        1,
    );

    assert_eq!(
        layer_map
            .assignment_for_node("rasterast-paper")
            .expect("protein assignment should exist")
            .layer,
        ZanistarastKnowledgeLayer::Protein,
    );
}

#[test]
fn rejects_incomplete_and_duplicate_layer_assignments() {
    let mut layer_map = KnowledgeLayerMap::new();

    assert!(!layer_map.assign(
        KnowledgeLayerAssignment::new(
            "",
            ZanistarastKnowledgeLayer::Dna,
            "Missing node identifier.",
        ),
    ));

    assert!(!layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun",
            ZanistarastKnowledgeLayer::Dna,
            " ",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn çekirdek kavramdır.",
        ),
    ));

    assert!(!layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun",
            ZanistarastKnowledgeLayer::Protein,
            "The same node cannot receive a second layer.",
        ),
    ));

    assert_eq!(layer_map.assignment_count(), 1);
}
  #[test]
    fn validates_layer_assignments_against_knowledge_map() {
        let knowledge_map = KnowledgeMapReport {
            maps: vec![DomainKnowledgeMap {
                domain: ZanistarastDomain::Hebun,
                nodes: vec![
                    KnowledgeNode {
                        id: "hebun-main".to_string(),
                        relative_path:
                            PathBuf::from(
                                "papers/hebun-main.md",
                            ),
                        title:
                            Some("Hebûn".to_string()),
                        readiness_score: 90,
                        maturity_level:
                            ArticleMaturityLevel::StrongCandidate,
                    },
                ],
                relations: Vec::new(),
            }],
        };

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-main",
                ZanistarastKnowledgeLayer::Dna,
                "Hebûn çekirdek kavramdır.",
            ),
        ));

        let validation =
            layer_map.validate_against(
                &knowledge_map,
            );

        assert!(validation.is_valid());

        assert_eq!(
            validation.assignment_count,
            1,
        );

        assert_eq!(
            validation.unknown_node_count(),
            0,
        );
    assert_eq!(
    validation.knowledge_node_count,
    1,
);

assert_eq!(
    validation.unassigned_node_count(),
    0,
);

assert!(!validation.has_unassigned_nodes());

    }

    #[test]
    fn reports_layer_assignment_for_unknown_node() {
        let knowledge_map = KnowledgeMapReport {
            maps: Vec::new(),
        };

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "missing-node",
                ZanistarastKnowledgeLayer::Protein,
                "Makale çıktısı olduğu düşünülüyor.",
            ),
        ));

        let validation =
            layer_map.validate_against(
                &knowledge_map,
            );

        assert!(!validation.is_valid());

        assert_eq!(
            validation.unknown_node_count(),
            1,
        );

        assert_eq!(
            validation.unknown_node_ids,
            vec!["missing-node".to_string()],
        );

        assert_eq!(
    validation.knowledge_node_count,
    0,
);

assert_eq!(
    validation.unassigned_node_count(),
    0,
);
    }
#[test]
fn reports_knowledge_nodes_without_layer_assignment() {
    let knowledge_map = KnowledgeMapReport {
        maps: vec![DomainKnowledgeMap {
            domain: ZanistarastDomain::Hebun,
            nodes: vec![
                KnowledgeNode {
                    id: "hebun-core".to_string(),
                    relative_path:
                        PathBuf::from(
                            "papers/hebun-core.md",
                        ),
                    title:
                        Some("Hebûn Çekirdeği".to_string()),
                    readiness_score: 95,
                    maturity_level:
                        ArticleMaturityLevel::StrongCandidate,
                },
                KnowledgeNode {
                    id: "hebun-process".to_string(),
                    relative_path:
                        PathBuf::from(
                            "papers/hebun-process.md",
                        ),
                    title:
                        Some("Hebûn Süreci".to_string()),
                    readiness_score: 80,
                    maturity_level:
                        ArticleMaturityLevel::DevelopingDraft,
                },
            ],
            relations: Vec::new(),
        }],
    };

    let mut layer_map = KnowledgeLayerMap::new();

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-core",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn çekirdek kavramdır.",
        ),
    ));

    let validation =
        layer_map.validate_against(&knowledge_map);

    assert!(!validation.is_valid());

    assert_eq!(
        validation.assignment_count,
        1,
    );

    assert_eq!(
        validation.knowledge_node_count,
        2,
    );

    assert_eq!(
        validation.unknown_node_count(),
        0,
    );

    assert_eq!(
        validation.unassigned_node_count(),
        1,
    );

    assert!(validation.has_unassigned_nodes());

    assert_eq!(
        validation.unassigned_node_ids,
        vec!["hebun-process".to_string()],
    );
}
#[test]
fn registers_zanistarast_dna_knowledge_types() {
    let mut dna_map = DnaKnowledgeMap::new();

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::CorePrinciple,
            "Hebûn, Zanistarast’ın değişmez çekirdek ilkesidir.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "rasterast-concept",
            ZanistarastDnaKind::Concept,
            "Rasterast, doğrulama katmanının temel kavramıdır.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "heksa-nizam-axiom",
            ZanistarastDnaKind::Axiom,
            "Heksa Nizam için biçimsel bir kurucu aksiyomdur.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "revelation-judgment",
            ZanistarastDnaKind::EpistemicJudgment,
            "Vahiy, Zanistarast epistemik düzeninde en yüksek ölçüdür.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "mudebbir-decision",
            ZanistarastDnaKind::OfficialDecision,
            "Müdebbir tarafından verilmiş resmî karardır.",
            true,
        ),
    ));

    assert_eq!(dna_map.record_count(), 5);

    assert_eq!(
        dna_map
            .records_of_kind(
                ZanistarastDnaKind::Axiom,
            )
            .len(),
        1,
    );

    assert!(dna_map.contains_node("hebun-core"));

    assert_eq!(
        dna_map
            .record_for_node("mudebbir-decision")
            .expect("official decision should exist")
            .kind,
        ZanistarastDnaKind::OfficialDecision,
    );
}

#[test]
fn rejects_invalid_and_duplicate_dna_records() {
    let mut dna_map = DnaKnowledgeMap::new();

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "",
            ZanistarastDnaKind::Concept,
            "Missing node identifier.",
            true,
        ),
    ));

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "temporary-rule",
            ZanistarastDnaKind::CorePrinciple,
            "DNA kaydı değişmez olmalıdır.",
            false,
        ),
    ));

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "empty-rationale",
            ZanistarastDnaKind::Axiom,
            " ",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::CorePrinciple,
            "Hebûn çekirdek ilkedir.",
            true,
        ),
    ));

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::Concept,
            "Aynı düğüm ikinci kez kaydedilemez.",
            true,
        ),
    ));

    assert_eq!(dna_map.record_count(), 1);
}
#[test]
fn registers_zanistarast_rna_knowledge_types() {
    let mut rna_map = RnaKnowledgeMap::new();

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "prepare-rasterast-paper",
            ZanistarastRnaKind::Task,
            "Rasterast makalesini hazırlama görevidir.",
            vec![
                "rasterast-concept".to_string(),
            ],
            vec![
                "rasterast-paper".to_string(),
            ],
        ),
    ));

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "academic-publication-process",
            ZanistarastRnaKind::Process,
            "Akademik analizi doğrulanmış yayına dönüştürür.",
            vec![
                "verified-academic-analysis".to_string(),
            ],
            vec![
                "publication-package".to_string(),
            ],
        ),
    ));

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "dna-to-article-rule",
            ZanistarastRnaKind::TransformationRule,
            "DNA çekirdek bilgisini makale planına dönüştürür.",
            vec![
                "hebun-core".to_string(),
            ],
            vec![
                "hebun-article-plan".to_string(),
            ],
        ),
    ));

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "repository-memory-transfer",
            ZanistarastRnaKind::KnowledgeTransfer,
            "Depo hafızasını akademik üretim bağlamına taşır.",
            vec![
                "repository-memory".to_string(),
            ],
            vec![
                "academic-context".to_string(),
            ],
        ),
    ));

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "rasterast-verification-request",
            ZanistarastRnaKind::VerificationRequest,
            "Akademik çıktıyı Rasterast doğrulamasına gönderir.",
            vec![
                "article-draft".to_string(),
            ],
            vec![
                "verified-article".to_string(),
            ],
        ),
    ));

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "mudebbir-publication-approval",
            ZanistarastRnaKind::ApprovalRequest,
            "Gerçek yayın öncesinde Müdebbir onayı ister.",
            vec![
                "verified-publication-package".to_string(),
            ],
            vec![
                "approved-publication-package".to_string(),
            ],
        ),
    ));

    assert_eq!(rna_map.record_count(), 6);

    assert_eq!(
        rna_map
            .records_of_kind(
                ZanistarastRnaKind::Task,
            )
            .len(),
        1,
    );

    assert_eq!(
        rna_map
            .records_using_source(
                "rasterast-concept",
            )
            .len(),
        1,
    );

    assert_eq!(
        rna_map
            .records_producing_target(
                "publication-package",
            )
            .len(),
        1,
    );

    assert!(
        rna_map.contains_node(
            "mudebbir-publication-approval",
        ),
    );
}

#[test]
fn rejects_invalid_and_duplicate_rna_records() {
    let mut rna_map = RnaKnowledgeMap::new();

    assert!(!rna_map.register(
        RnaKnowledgeRecord::new(
            "",
            ZanistarastRnaKind::Task,
            "Missing node identifier.",
            Vec::new(),
            Vec::new(),
        ),
    ));

    assert!(!rna_map.register(
        RnaKnowledgeRecord::new(
            "empty-rationale",
            ZanistarastRnaKind::Process,
            " ",
            Vec::new(),
            Vec::new(),
        ),
    ));

    assert!(!rna_map.register(
        RnaKnowledgeRecord::new(
            "invalid-source",
            ZanistarastRnaKind::TransformationRule,
            "Kaynak düğüm kimliği boş olamaz.",
            vec![" ".to_string()],
            vec!["target-node".to_string()],
        ),
    ));

    assert!(!rna_map.register(
        RnaKnowledgeRecord::new(
            "invalid-target",
            ZanistarastRnaKind::KnowledgeTransfer,
            "Hedef düğüm kimliği boş olamaz.",
            vec!["source-node".to_string()],
            vec!["".to_string()],
        ),
    ));

    assert!(rna_map.register(
        RnaKnowledgeRecord::new(
            "article-process",
            ZanistarastRnaKind::Process,
            "Makale üretim sürecidir.",
            vec!["article-plan".to_string()],
            vec!["article-draft".to_string()],
        ),
    ));

    assert!(!rna_map.register(
        RnaKnowledgeRecord::new(
            "article-process",
            ZanistarastRnaKind::Task,
            "Aynı düğüm ikinci kez kaydedilemez.",
            Vec::new(),
            Vec::new(),
        ),
    ));

    assert_eq!(rna_map.record_count(), 1);
}

#[test]
fn rna_record_reports_sources_and_targets() {
    let record = RnaKnowledgeRecord::new(
        "dna-to-protein-process",
        ZanistarastRnaKind::TransformationRule,
        "DNA bilgisini somut akademik çıktıya dönüştürür.",
        vec![
            "hebun-core".to_string(),
            "rasterast-concept".to_string(),
        ],
        vec![
            "hebun-paper".to_string(),
            "rasterast-report".to_string(),
        ],
    );

    assert!(record.is_complete());

    assert!(
        record.uses_source_node("hebun-core"),
    );

    assert!(
        record.uses_source_node(
            "rasterast-concept",
        ),
    );

    assert!(
        record.produces_target_node(
            "hebun-paper",
        ),
    );

    assert!(
        !record.produces_target_node(
            "unknown-output",
        ),
    );
}
#[test]
fn registers_zanistarast_protein_knowledge_types() {
    let mut protein_map = ProteinKnowledgeMap::new();

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "rasterast-paper",
            ZanistarastProteinKind::Article,
            "Rasterast hakkındaki somut akademik makaledir.",
            vec![
                "rasterast-concept".to_string(),
                "prepare-rasterast-paper".to_string(),
            ],
            Some(PathBuf::from(
                "papers/rasterast.md",
            )),
            true,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "repository-graph-module",
            ZanistarastProteinKind::CodeModule,
            "Depolar arasındaki ilişkileri yöneten kod modülüdür.",
            vec![
                "repository-graph-process".to_string(),
            ],
            Some(PathBuf::from(
                "reference-implementation/mira/src/repository_graph.rs",
            )),
            true,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "rasterast-verification-report",
            ZanistarastProteinKind::Report,
            "Rasterast doğrulama sürecinin rapor çıktısıdır.",
            vec![
                "rasterast-verification-request".to_string(),
            ],
            Some(PathBuf::from(
                "reports/rasterast-verification.md",
            )),
            false,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "zenodo-publication-package",
            ZanistarastProteinKind::PublicationPackage,
            "Zenodo için hazırlanmış yayın paketidir.",
            vec![
                "academic-publication-process".to_string(),
            ],
            Some(PathBuf::from(
                "dist/zenodo-package",
            )),
            false,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "zanistarast-website",
            ZanistarastProteinKind::Application,
            "Zanistarast yayınlarının görünür uygulama çıktısıdır.",
            vec![
                "website-publication-process".to_string(),
            ],
            None,
            true,
        ),
    ));

    assert_eq!(protein_map.record_count(), 5);

    assert_eq!(
        protein_map
            .records_of_kind(
                ZanistarastProteinKind::Article,
            )
            .len(),
        1,
    );

    assert_eq!(
        protein_map
            .records_using_source(
                "academic-publication-process",
            )
            .len(),
        1,
    );

    assert_eq!(
        protein_map.verified_records().len(),
        3,
    );

    assert_eq!(
        protein_map.unverified_records().len(),
        2,
    );

    assert!(
        protein_map.contains_node(
            "rasterast-paper",
        ),
    );
}

#[test]
fn rejects_invalid_and_duplicate_protein_records() {
    let mut protein_map = ProteinKnowledgeMap::new();

    assert!(!protein_map.register(
        ProteinKnowledgeRecord::new(
            "",
            ZanistarastProteinKind::Article,
            "Missing node identifier.",
            Vec::new(),
            None,
            false,
        ),
    ));

    assert!(!protein_map.register(
        ProteinKnowledgeRecord::new(
            "empty-rationale",
            ZanistarastProteinKind::Report,
            " ",
            Vec::new(),
            None,
            false,
        ),
    ));

    assert!(!protein_map.register(
        ProteinKnowledgeRecord::new(
            "invalid-source",
            ZanistarastProteinKind::CodeModule,
            "Kaynak düğüm kimliği boş olamaz.",
            vec![" ".to_string()],
            None,
            false,
        ),
    ));

    assert!(!protein_map.register(
        ProteinKnowledgeRecord::new(
            "invalid-path",
            ZanistarastProteinKind::PublicationPackage,
            "Boş göreli yol kabul edilmez.",
            Vec::new(),
            Some(PathBuf::new()),
            false,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "hebun-paper",
            ZanistarastProteinKind::Article,
            "Hebûn akademik makalesidir.",
            vec!["hebun-article-process".to_string()],
            Some(PathBuf::from(
                "papers/hebun.md",
            )),
            false,
        ),
    ));

    assert!(!protein_map.register(
        ProteinKnowledgeRecord::new(
            "hebun-paper",
            ZanistarastProteinKind::Report,
            "Aynı düğüm ikinci kez kaydedilemez.",
            Vec::new(),
            None,
            false,
        ),
    ));

    assert_eq!(protein_map.record_count(), 1);
}

#[test]
fn protein_record_reports_source_path_and_verification() {
    let record = ProteinKnowledgeRecord::new(
        "rasterast-paper",
        ZanistarastProteinKind::Article,
        "Rasterast akademik çıktısıdır.",
        vec![
            "rasterast-concept".to_string(),
            "article-production-process".to_string(),
        ],
        Some(PathBuf::from(
            "papers/rasterast.md",
        )),
        true,
    );

    assert!(record.is_complete());

    assert!(
        record.uses_source_node(
            "rasterast-concept",
        ),
    );

    assert!(
        !record.uses_source_node(
            "unknown-source",
        ),
    );

    assert!(record.has_relative_path());
    assert!(record.is_verified());

    assert_eq!(
        record.relative_path.as_deref(),
        Some(std::path::Path::new(
            "papers/rasterast.md",
        )),
    );
}
#[test]
fn registers_zanistarast_protein_knowledge_types() {
    let mut protein_map = ProteinKnowledgeMap::new();

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "rasterast-paper",
            ZanistarastProteinKind::Article,
            "Rasterast hakkındaki somut akademik makaledir.",
            vec![
                "rasterast-concept".to_string(),
                "prepare-rasterast-paper".to_string(),
            ],
            Some(PathBuf::from(
                "papers/rasterast.md",
            )),
            true,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "repository-graph-module",
            ZanistarastProteinKind::CodeModule,
            "Depolar arasındaki ilişkileri yöneten kod modülüdür.",
            vec![
                "repository-graph-process".to_string(),
            ],
            Some(PathBuf::from(
                "reference-implementation/mira/src/repository_graph.rs",
            )),
            true,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "rasterast-verification-report",
            ZanistarastProteinKind::Report,
            "Rasterast doğrulama sürecinin rapor çıktısıdır.",
            vec![
                "rasterast-verification-request".to_string(),
            ],
            Some(PathBuf::from(
                "reports/rasterast-verification.md",
            )),
            false,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "zenodo-publication-package",
            ZanistarastProteinKind::PublicationPackage,
            "Zenodo için hazırlanmış yayın paketidir.",
            vec![
                "academic-publication-process".to_string(),
            ],
            Some(PathBuf::from(
                "dist/zenodo-package",
            )),
            false,
        ),
    ));

    assert!(protein_map.register(
        ProteinKnowledgeRecord::new(
            "zanistarast-website",
            ZanistarastProteinKind::Application,
            "Zanistarast yayınlarının görünür uygulama çıktısıdır.",
            vec![
                "website-publication-process".to_string(),
            ],
            None,
            true,
        ),
    ));

    assert_eq!(protein_map.record_count(), 5);

    assert_eq!(
        protein_map
            .records_of_kind(
                ZanistarastProteinKind::Article,
            )
            .len(),
        1,
    );

    assert_eq!(
        protein_map
            .records_using_source(
                "academic-publication-process",
            )
            .len(),
        1,
    );

    assert_eq!(
        protein_map.verified_records().len(),
        3,
    );

    assert_eq!(
        protein_map.unverified_records().len(),
        2,
    );

    assert!(
        protein_map.contains_node(
            "rasterast-paper",
        ),
    );
}

#[test]
fn validates_complete_dna_rna_protein_architecture() {
    let mut architecture =
        ZanistarastKnowledgeArchitecture::new();

    assert!(architecture.dna.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::CorePrinciple,
            "Hebûn değişmez çekirdek ilkedir.",
            true,
        ),
    ));

    assert!(architecture.rna.register(
        RnaKnowledgeRecord::new(
            "hebun-article-process",
            ZanistarastRnaKind::Process,
            "Hebûn bilgisini makale çıktısına dönüştürür.",
            vec!["hebun-core".to_string()],
            vec!["hebun-paper".to_string()],
        ),
    ));

    assert!(architecture.protein.register(
        ProteinKnowledgeRecord::new(
            "hebun-paper",
            ZanistarastProteinKind::Article,
            "Hebûn hakkındaki somut akademik makaledir.",
            vec![
                "hebun-core".to_string(),
                "hebun-article-process".to_string(),
            ],
            Some(PathBuf::from(
                "papers/hebun.md",
            )),
            false,
        ),
    ));

    let mut layer_map = KnowledgeLayerMap::new();

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-core",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn çekirdek ilkedir.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-article-process",
            ZanistarastKnowledgeLayer::Rna,
            "Makale üretim sürecidir.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-paper",
            ZanistarastKnowledgeLayer::Protein,
            "Somut akademik çıktıdır.",
        ),
    ));

    let validation =
        architecture.validate_layer_alignment(
            &layer_map,
        );

    assert!(validation.is_valid());

    assert_eq!(
        architecture.total_record_count(),
        3,
    );

    assert!(!architecture.is_empty());

    assert_eq!(
        architecture.layer_for_node(
            "hebun-core",
        ),
        Some(ZanistarastKnowledgeLayer::Dna),
    );

    assert_eq!(
        architecture.layer_for_node(
            "hebun-article-process",
        ),
        Some(ZanistarastKnowledgeLayer::Rna),
    );

    assert_eq!(
        architecture.layer_for_node(
            "hebun-paper",
        ),
        Some(ZanistarastKnowledgeLayer::Protein),
    );

    assert_eq!(
        validation.assignment_count,
        3,
    );

    assert_eq!(
        validation.detailed_record_count,
        3,
    );
}

#[test]
fn reports_invalid_knowledge_architecture_alignment() {
    let mut architecture =
        ZanistarastKnowledgeArchitecture::new();

    assert!(architecture.dna.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::CorePrinciple,
            "Hebûn değişmez çekirdek ilkedir.",
            true,
        ),
    ));

    assert!(architecture.protein.register(
        ProteinKnowledgeRecord::new(
            "rasterast-paper",
            ZanistarastProteinKind::Article,
            "Rasterast akademik makalesidir.",
            Vec::new(),
            Some(PathBuf::from(
                "papers/rasterast.md",
            )),
            false,
        ),
    ));

    let mut layer_map = KnowledgeLayerMap::new();

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-core",
            ZanistarastKnowledgeLayer::Protein,
            "Bilerek yanlış katmana atanmıştır.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "missing-process",
            ZanistarastKnowledgeLayer::Rna,
            "Ayrıntılı RNA kaydı bulunmamaktadır.",
        ),
    ));

    let validation =
        architecture.validate_layer_alignment(
            &layer_map,
        );

    assert!(!validation.is_valid());

    assert_eq!(
        validation.mismatched_layer_nodes,
        vec!["hebun-core".to_string()],
    );

    assert_eq!(
        validation.missing_detailed_records,
        vec!["missing-process".to_string()],
    );

    assert_eq!(
        validation.unassigned_detailed_nodes,
        vec!["rasterast-paper".to_string()],
    );

    assert_eq!(
        validation.mismatched_layer_count(),
        1,
    );

    assert_eq!(
        validation.missing_detailed_record_count(),
        1,
    );

    assert_eq!(
        validation
            .unassigned_detailed_node_count(),
        1,
    );
}

}





