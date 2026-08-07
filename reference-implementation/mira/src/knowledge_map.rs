use crate::article_candidate_analysis::ArticleMaturityLevel;
use crate::article_inventory::ZanistarastDomain;
use crate::topic_clustering::{
    ClusteredArticle,
    TopicClusteringReport,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Bilgi haritasındaki tek bir bilimsel içerik düğümü.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeNode {
    pub id: String,
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub readiness_score: u8,
    pub maturity_level: ArticleMaturityLevel,
}

impl KnowledgeNode {
    /// Bilgi düğümünün zorunlu alanlarının
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.relative_path.as_os_str().is_empty()
    }
}

/// Aynı bilgi alanındaki iki düğüm arasındaki ilişki.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: KnowledgeRelationType,
}

impl KnowledgeRelation {
    /// İlişkinin zorunlu bilgilerinin eksiksiz
    /// ve kaynak ile hedefin farklı olup
    /// olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.source_id.trim().is_empty()
            && !self.target_id.trim().is_empty()
            && self.source_id != self.target_id
    }
}

/// Bilgi haritasında kullanılabilecek ilişki türleri.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum KnowledgeRelationType {
    SameDomain,
    PossibleContinuation,
    RequiresReview,
}

/// Bir bilgi düğümünün Zanistarast bilgi
/// mimarisindeki işlevsel katmanını belirtir.
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
    /// aksiyomlar, epistemik hükümler
    /// ve resmî kararlar.
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

/// Bir bilgi düğümünün Zanistarast DNA
/// katmanındaki ayrıntılı ve gerekçeli kaydıdır.
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
    /// ve DNA ilkeleriyle uyumlu olup olmadığını
    /// bildirir.
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

    /// DNA bilgi kayıtlarını salt okunur
    /// biçimde döndürür.
    pub fn records(&self) -> &[DnaKnowledgeRecord] {
        &self.records
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

    /// Belirtilen DNA kaydını kaldırır.
    ///
    /// Değişmez bilgi üzerinde gerçek silme kararı
    /// daha sonra Müdebbir onay kapısına bağlanacaktır.
    pub fn remove_record(
        &mut self,
        node_id: &str,
    ) -> Option<DnaKnowledgeRecord> {
        let position = self.records.iter().position(|record| {
            record.node_id == node_id
        })?;

        Some(self.records.remove(position))
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

/// Bir bilgi düğümünün Zanistarast RNA
/// katmanındaki ayrıntılı ve gerekçeli süreç kaydıdır.
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

    /// RNA kaydının herhangi bir kaynak düğüme
    /// bağlı olup olmadığını bildirir.
    pub fn has_sources(&self) -> bool {
        !self.source_node_ids.is_empty()
    }

    /// RNA kaydının herhangi bir hedef düğüm
    /// üretip üretmediğini bildirir.
    pub fn has_targets(&self) -> bool {
        !self.target_node_ids.is_empty()
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

    /// RNA kayıtlarını salt okunur biçimde döndürür.
    pub fn records(&self) -> &[RnaKnowledgeRecord] {
        &self.records
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

    /// Kaynak düğümü bulunmayan RNA kayıtlarını
    /// döndürür.
    pub fn records_without_sources(
        &self,
    ) -> Vec<&RnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| !record.has_sources())
            .collect()
    }

    /// Hedef düğümü bulunmayan RNA kayıtlarını
    /// döndürür.
    pub fn records_without_targets(
        &self,
    ) -> Vec<&RnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| !record.has_targets())
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

    /// Belirtilen RNA kaydını kaldırır.
    pub fn remove_record(
        &mut self,
        node_id: &str,
    ) -> Option<RnaKnowledgeRecord> {
        let position = self.records.iter().position(|record| {
            record.node_id == node_id
        })?;

        Some(self.records.remove(position))
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
                .is_none_or(|path| !path.as_os_str().is_empty())
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
    /// düğümünden üretildiğini bildirir.
    pub fn uses_source_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.source_node_ids
            .iter()
            .any(|source| source == node_id)
    }

    /// Protein çıktısının göreli yolunun
    /// tanımlı olup olmadığını bildirir.
    pub fn has_relative_path(&self) -> bool {
        self.relative_path
            .as_ref()
            .is_some_and(|path| !path.as_os_str().is_empty())
    }

    /// Protein çıktısının doğrulanmış
    /// olup olmadığını bildirir.
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

    /// Yeni bir Protein kaydı ekler.
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

    /// Toplam kayıt sayısını döndürür.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Haritanın boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Salt okunur kayıt listesini döndürür.
    pub fn records(&self) -> &[ProteinKnowledgeRecord] {
        &self.records
    }

    /// Düğüme ait Protein kaydını döndürür.
    pub fn record_for_node(
        &self,
        node_id: &str,
    ) -> Option<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .find(|record| record.node_id == node_id)
    }

    /// Türe göre kayıtları döndürür.
    pub fn records_of_kind(
        &self,
        kind: ZanistarastProteinKind,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_kind(kind))
            .collect()
    }

    /// Kaynak düğüme göre kayıtları döndürür.
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

    /// Doğrulanmış kayıtları döndürür.
    pub fn verified_records(
        &self,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_verified())
            .collect()
    }

    /// Doğrulanmamış kayıtları döndürür.
    pub fn unverified_records(
        &self,
    ) -> Vec<&ProteinKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| !record.is_verified())
            .collect()
    }

    /// Düğümün Protein katmanında kayıtlı
    /// olup olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.record_for_node(node_id).is_some()
    }

    /// Bir Protein kaydını kaldırır.
    pub fn remove_record(
        &mut self,
        node_id: &str,
    ) -> Option<ProteinKnowledgeRecord> {
        let position = self.records.iter().position(|record| {
            record.node_id == node_id
        })?;

        Some(self.records.remove(position))
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
    /// Yeni katman ataması oluşturur.
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

    /// Atamanın geçerli olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
    }
}

/// Bilgi düğümlerinin DNA–RNA–Protein
/// katmanlarına atanmış görünümüdür.
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
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn assignment_count(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    pub fn assignments(
        &self,
    ) -> &[KnowledgeLayerAssignment] {
        &self.assignments
    }

    pub fn assignment_for_node(
        &self,
        node_id: &str,
    ) -> Option<&KnowledgeLayerAssignment> {
        self.assignments
            .iter()
            .find(|assignment| assignment.node_id == node_id)
    }

    pub fn assignments_in_layer(
        &self,
        layer: ZanistarastKnowledgeLayer,
    ) -> Vec<&KnowledgeLayerAssignment> {
        self.assignments
            .iter()
            .filter(|assignment| assignment.layer == layer)
            .collect()
    }

    pub fn has_assignment(
        &self,
        node_id: &str,
    ) -> bool {
        self.assignment_for_node(node_id).is_some()
    }

    pub fn assigned_nodes(
        &self,
    ) -> Vec<&str> {
        self.assignments
            .iter()
            .map(|assignment| assignment.node_id.as_str())
            .collect()
    }

    pub fn layer_distribution(
        &self,
    ) -> (usize, usize, usize) {
        (
            self.assignments_in_layer(
                ZanistarastKnowledgeLayer::Dna,
            )
            .len(),
            self.assignments_in_layer(
                ZanistarastKnowledgeLayer::Rna,
            )
            .len(),
            self.assignments_in_layer(
                ZanistarastKnowledgeLayer::Protein,
            )
            .len(),
        )
    }
}

/// Zanistarast DNA–RNA–Protein bilgi mimarisinin
/// birleşik modelidir.
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_record_count(&self) -> usize {
        self.dna.record_count()
            + self.rna.record_count()
            + self.protein.record_count()
    }

    pub fn is_empty(&self) -> bool {
        self.dna.is_empty()
            && self.rna.is_empty()
            && self.protein.is_empty()
    }

    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.dna.contains_node(node_id)
            || self.rna.contains_node(node_id)
            || self.protein.contains_node(node_id)
    }

    pub fn layer_for_node(
        &self,
        node_id: &str,
    ) -> Option<ZanistarastKnowledgeLayer> {
        if self.dna.contains_node(node_id) {
            return Some(
                ZanistarastKnowledgeLayer::Dna,
            );
        }

        if self.rna.contains_node(node_id) {
            return Some(
                ZanistarastKnowledgeLayer::Rna,
            );
        }

        if self.protein.contains_node(node_id) {
            return Some(
                ZanistarastKnowledgeLayer::Protein,
            );
        }

        None
    }

   /// DNA–RNA–Protein ayrıntılı kayıtlarının genel
    /// katman atamalarıyla uyumluluğunu doğrular.
    pub fn validate_layer_alignment(
        &self,
        layer_map: &KnowledgeLayerMap,
    ) -> KnowledgeArchitectureValidationReport {
        let mut report = KnowledgeArchitectureValidationReport {
            assignment_count: layer_map.assignment_count(),
            detailed_record_count: self.total_record_count(),
            ..Default::default()
        };

        for assignment in layer_map.assignments() {
            match self.layer_for_node(
                &assignment.node_id,
            ) {
                None => {
                    report
                        .missing_detailed_records
                        .push(assignment.node_id.clone());
                }

                Some(layer)
                    if layer != assignment.layer =>
                {
                    report
                        .mismatched_layer_nodes
                        .push(assignment.node_id.clone());
                }

                Some(_) => {}
            }
        }

        for record in self.dna.records() {
            if !layer_map.has_assignment(
                &record.node_id,
            ) {
                report
                    .unassigned_detailed_nodes
                    .push(record.node_id.clone());
            }
        }

        for record in self.rna.records() {
            if !layer_map.has_assignment(
                &record.node_id,
            ) {
                report
                    .unassigned_detailed_nodes
                    .push(record.node_id.clone());
            }
        }

        for record in self.protein.records() {
            if !layer_map.has_assignment(
                &record.node_id,
            ) {
                report
                    .unassigned_detailed_nodes
                    .push(record.node_id.clone());
            }
        }

        report.missing_detailed_records.sort();
        report.missing_detailed_records.dedup();

        report.mismatched_layer_nodes.sort();
        report.mismatched_layer_nodes.dedup();

        report.unassigned_detailed_nodes.sort();
        report.unassigned_detailed_nodes.dedup();

        report
    }

     /// DNA–RNA–Protein kayıtları arasındaki kaynak
    /// ve hedef bağlantılarının mevcut düğümlere
    /// dayanıp dayanmadığını doğrular.
    ///
    /// RNA kaynakları DNA, RNA veya Protein
    /// katmanındaki mevcut bir düğüme bağlanmalıdır.
    ///
    /// RNA hedefleri RNA veya Protein
    /// katmanındaki mevcut bir düğüme bağlanmalıdır.
    ///
    /// Protein kaynakları DNA, RNA veya Protein
    /// katmanındaki mevcut bir düğüme bağlanmalıdır.
    pub fn validate_knowledge_chain(
        &self,
    ) -> KnowledgeChainValidationReport {
        let mut report = KnowledgeChainValidationReport {
            rna_record_count: self.rna.record_count(),
            protein_record_count:
                self.protein.record_count(),
            ..Default::default()
        };

        for record in self.rna.records() {
            for source_node_id in
                &record.source_node_ids
            {
                if !self.contains_node(source_node_id) {
                    report
                        .missing_rna_source_nodes
                        .push(format!(
                            "{}:{}",
                            record.node_id,
                            source_node_id,
                        ));
                }
            }

            for target_node_id in
                &record.target_node_ids
            {
                let target_exists =
                    self.rna.contains_node(
                        target_node_id,
                    )
                    || self.protein.contains_node(
                        target_node_id,
                    );

                if !target_exists {
                    report
                        .missing_rna_target_nodes
                        .push(format!(
                            "{}:{}",
                            record.node_id,
                            target_node_id,
                        ));
                }
            }
        }

        for record in self.protein.records() {
            for source_node_id in
                &record.source_node_ids
            {
                if !self.contains_node(source_node_id) {
                    report
                        .missing_protein_source_nodes
                        .push(format!(
                            "{}:{}",
                            record.node_id,
                            source_node_id,
                        ));
                }
            }
        }

        report.missing_rna_source_nodes.sort();
        report.missing_rna_source_nodes.dedup();

        report.missing_rna_target_nodes.sort();
        report.missing_rna_target_nodes.dedup();

        report
            .missing_protein_source_nodes
            .sort();
        report
            .missing_protein_source_nodes
            .dedup();

        report
    }

    /// Zanistarast bilgi mimarisinin katman
    /// atamalarını ve DNA–RNA–Protein zincirini
    /// tek bir raporda doğrular.
pub fn validate_architecture(
        &self,
        layer_map: &KnowledgeLayerMap,
    ) -> KnowledgeArchitectureHealthReport {
        KnowledgeArchitectureHealthReport {
            layer_alignment:
                self.validate_layer_alignment(layer_map),
            knowledge_chain:
                self.validate_knowledge_chain(),
        }
    }
  }
/// Katman doğrulama raporu.
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
    /// kaydı bulunmayan düğüm sayısını döndürür.
    pub fn missing_count(&self) -> usize {
        self.missing_detailed_records.len()
    }

    /// Atanan katman ile gerçek ayrıntılı katmanı
    /// uyuşmayan düğüm sayısını döndürür.
    pub fn mismatch_count(&self) -> usize {
        self.mismatched_layer_nodes.len()
    }

    /// Ayrıntılı kaydı bulunduğu hâlde genel
    /// katman ataması olmayan düğüm sayısını döndürür.
    pub fn unassigned_count(&self) -> usize {
        self.unassigned_detailed_nodes.len()
    }
}

/// Zanistarast DNA–RNA–Protein bilgi akışındaki
/// eksik kaynak ve hedef bağlantılarını raporlar.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]

pub struct KnowledgeChainValidationReport {
    pub rna_record_count: usize,
    pub protein_record_count: usize,
    pub missing_rna_source_nodes: Vec<String>,
    pub missing_rna_target_nodes: Vec<String>,
    pub missing_protein_source_nodes: Vec<String>,
}

impl KnowledgeChainValidationReport {
    /// DNA–RNA–Protein bilgi zincirindeki bütün
    /// kaynak ve hedef bağlantılarının geçerli
    /// olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.missing_rna_source_nodes.is_empty()
            && self.missing_rna_target_nodes.is_empty()
            && self
                .missing_protein_source_nodes
                .is_empty()
    }

    /// RNA kayıtlarında bulunamayan kaynak
    /// bağlantılarının sayısını döndürür.
    pub fn missing_rna_source_count(
        &self,
    ) -> usize {
        self.missing_rna_source_nodes.len()
    }

    /// RNA kayıtlarında bulunamayan hedef
    /// bağlantılarının sayısını döndürür.
    pub fn missing_rna_target_count(
        &self,
    ) -> usize {
        self.missing_rna_target_nodes.len()
    }

    /// Protein kayıtlarında bulunamayan kaynak
    /// bağlantılarının sayısını döndürür.
    pub fn missing_protein_source_count(
        &self,
    ) -> usize {
        self.missing_protein_source_nodes.len()
    }

    /// Zincirde bulunan toplam eksik bağlantı
    /// sayısını döndürür.
    pub fn missing_link_count(&self) -> usize {
        self.missing_rna_source_count()
            + self.missing_rna_target_count()
            + self.missing_protein_source_count()
    }
}
/// Zanistarast bilgi mimarisinin katman ataması
/// ve bilgi zinciri sonuçlarını birleştiren rapordur.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeArchitectureHealthReport {
    pub layer_alignment:
        KnowledgeArchitectureValidationReport,
    pub knowledge_chain:
        KnowledgeChainValidationReport,
}

impl KnowledgeArchitectureHealthReport {
    /// Katman atamalarının ve bilgi zincirinin
    /// birlikte geçerli olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.layer_alignment.is_valid()
            && self.knowledge_chain.is_valid()
    }

    /// Katman atamalarında bulunan toplam
    /// sorun sayısını döndürür.
    pub fn layer_issue_count(&self) -> usize {
        self.layer_alignment.missing_count()
            + self.layer_alignment.mismatch_count()
            + self.layer_alignment.unassigned_count()
    }

    /// Bilgi zincirinde bulunan toplam eksik
    /// bağlantı sayısını döndürür.
    pub fn chain_issue_count(&self) -> usize {
        self.knowledge_chain.missing_link_count()
    }

    /// Birleşik mimaride bulunan toplam sorun
    /// sayısını döndürür.
    pub fn total_issue_count(&self) -> usize {
        self.layer_issue_count()
            + self.chain_issue_count()
    }

    /// Mimari raporunda herhangi bir sorun
    /// bulunup bulunmadığını bildirir.
    pub fn has_issues(&self) -> bool {
        self.total_issue_count() > 0
    }
 /// Katman doğrulaması ile bilgi zinciri
/// doğrulamasının birlikte başarılı olup
/// olmadığını bildirir.
pub fn is_healthy(&self) -> bool {
    self.layer_alignment.is_valid()
        && self.knowledge_chain.is_valid()
}
/// Mimaride tespit edilen toplam sorun sayısını döndürür.
pub fn issue_count(&self) -> usize {
    self.layer_alignment.missing_count()
        + self.layer_alignment.mismatch_count()
        + self.layer_alignment.unassigned_count()
        + self.knowledge_chain.missing_link_count()
}
/// Katman doğrulaması ile bilgi zinciri
/// doğrulamasının özet durumunu döndürür.
pub fn summary(&self) -> String {
    format!(
        "layer_valid={}, chain_valid={}, issues={}",
        self.layer_alignment.is_valid(),
        self.knowledge_chain.is_valid(),
        self.issue_count(),
    )
    }
       

/// Tek bir Zanistarast alanına ait bilgi haritası.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct DomainKnowledgeMap {
    pub domain: ZanistarastDomain,
    pub nodes: Vec<KnowledgeNode>,
    pub relations: Vec<KnowledgeRelation>,
}

impl DomainKnowledgeMap {
    /// Haritadaki düğüm sayısını döndürür.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Haritadaki ilişki sayısını döndürür.
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Belirtilen kimliğe sahip bilgi düğümünü döndürür.
    pub fn node(
        &self,
        node_id: &str,
    ) -> Option<&KnowledgeNode> {
        self.nodes
            .iter()
            .find(|node| node.id == node_id)
    }

    /// Belirtilen düğümle bağlantılı ilişkileri döndürür.
    pub fn relations_for_node(
        &self,
        node_id: &str,
    ) -> Vec<&KnowledgeRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.source_id == node_id
                    || relation.target_id == node_id
            })
            .collect()
    }
}

/// Tüm Zanistarast bilgi haritalarının birleşik raporu.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeMapReport {
    pub maps: Vec<DomainKnowledgeMap>,
}

impl KnowledgeMapReport {
    /// Rapordaki alan haritası sayısını döndürür.
    pub fn map_count(&self) -> usize {
        self.maps.len()
    }

    /// Bütün alanlardaki toplam düğüm sayısını döndürür.
    pub fn total_node_count(&self) -> usize {
        self.maps
            .iter()
            .map(DomainKnowledgeMap::node_count)
            .sum()
    }

    /// Bütün alanlardaki toplam ilişki sayısını döndürür.
    pub fn total_relation_count(&self) -> usize {
        self.maps
            .iter()
            .map(DomainKnowledgeMap::relation_count)
            .sum()
    }

    /// Belirtilen Zanistarast alanına ait haritayı döndürür.
    pub fn map_for_domain(
        &self,
        domain: &ZanistarastDomain,
    ) -> Option<&DomainKnowledgeMap> {
        self.maps
            .iter()
            .find(|map| &map.domain == domain)
    }

    /// Bütün alan haritaları içinde düğüm arar.
    pub fn node(
        &self,
        node_id: &str,
    ) -> Option<&KnowledgeNode> {
        self.maps
            .iter()
            .find_map(|map| map.node(node_id))
    }

    /// Belirtilen düğümün bilgi haritasında
    /// bulunup bulunmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.node(node_id).is_some()
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
    /// Yeni bir bilgi haritası oluşturucusu döndürür.
    pub fn new() -> Self {
        Self
    }

    /// Konu kümelerinden Zanistarast alan
    /// bilgi haritaları oluşturur.
    pub fn build(
        &self,
        clustering_report: &TopicClusteringReport,
    ) -> KnowledgeMapReport {
        let mut maps = clustering_report
            .clusters
            .iter()
            .filter(|cluster| {
                cluster.domain
                    != ZanistarastDomain::Unclassified
            })
            .map(|cluster| {
                let nodes = cluster
                    .articles
                    .iter()
                    .map(Self::node_from_article)
                    .collect::<Vec<_>>();

                let relations =
                    Self::build_relations(&nodes);

                DomainKnowledgeMap {
                    domain: cluster.domain.clone(),
                    nodes,
                    relations,
                }
            })
            .collect::<Vec<_>>();

        maps.sort_by(|left, right| {
            left.domain.cmp(&right.domain)
        });

        KnowledgeMapReport { maps }
    }

    fn node_from_article(
        article: &ClusteredArticle,
    ) -> KnowledgeNode {
        KnowledgeNode {
            id: Self::node_id(
                &article.relative_path,
            ),
            relative_path:
                article.relative_path.clone(),
            title: article.title.clone(),
            readiness_score:
                article.readiness_score,
            maturity_level:
                article.maturity_level.clone(),
        }
    }

    fn node_id(
        path: &std::path::Path,
    ) -> String {
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

        for (index, source) in
            nodes.iter().enumerate()
        {
            for target in
                nodes.iter().skip(index + 1)
            {
                let relation = KnowledgeRelation {
                    source_id: source.id.clone(),
                    target_id: target.id.clone(),
                    relation_type:
                        Self::relation_type(
                            source,
                            target,
                        ),
                };

                if relation.is_complete() {
                    relations.push(relation);
                }
            }
        }

        relations
    }

    fn relation_type(
        source: &KnowledgeNode,
        target: &KnowledgeNode,
    ) -> KnowledgeRelationType {
        if source.maturity_level
            == ArticleMaturityLevel::Fragment
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

    fn sample_knowledge_node(
        id: &str,
        relative_path: &str,
        readiness_score: u8,
        maturity_level: ArticleMaturityLevel,
    ) -> KnowledgeNode {
        KnowledgeNode {
            id: id.to_string(),
            relative_path:
                PathBuf::from(relative_path),
            title: Some(id.to_string()),
            readiness_score,
            maturity_level,
        }
    }

    #[test]
    fn builder_creates_domain_knowledge_maps() {
        let clustering_report =
            TopicClusteringReport {
                clusters: vec![
                    TopicCluster {
                        domain:
                            ZanistarastDomain::Hebun,
                        articles: vec![
                            ClusteredArticle {
                                relative_path:
                                    PathBuf::from(
                                        "papers/hebun-main.md",
                                    ),
                                title: Some(
                                    "Hebûn Ana Makalesi"
                                        .to_string(),
                                ),
                                readiness_score: 90,
                                maturity_level:
                                    ArticleMaturityLevel::
                                        StrongCandidate,
                            },
                            ClusteredArticle {
                                relative_path:
                                    PathBuf::from(
                                        "papers/hebun-method.md",
                                    ),
                                title: Some(
                                    "Hebûn Yöntemi"
                                        .to_string(),
                                ),
                                readiness_score: 80,
                                maturity_level:
                                    ArticleMaturityLevel::
                                        DevelopingDraft,
                            },
                        ],
                        total_readiness_score: 170,
                    },
                    TopicCluster {
                        domain:
                            ZanistarastDomain::Rabun,
                        articles: vec![
                            ClusteredArticle {
                                relative_path:
                                    PathBuf::from(
                                        "papers/rabun.md",
                                    ),
                                title: Some(
                                    "Rabûn Yönetim Modeli"
                                        .to_string(),
                                ),
                                readiness_score: 75,
                                maturity_level:
                                    ArticleMaturityLevel::
                                        DevelopingDraft,
                            },
                        ],
                        total_readiness_score: 75,
                    },
                ],
                total_clustered_articles: 3,
            };

        let report =
            KnowledgeMapBuilder::new().build(
                &clustering_report,
            );

        assert_eq!(report.map_count(), 2);
        assert_eq!(report.total_node_count(), 3);
        assert_eq!(
            report.total_relation_count(),
            1,
        );

        let hebun_map = report
            .map_for_domain(
                &ZanistarastDomain::Hebun,
            )
            .expect(
                "Hebûn knowledge map should exist",
            );

        assert_eq!(hebun_map.node_count(), 2);
        assert_eq!(
            hebun_map.relation_count(),
            1,
        );

        assert_eq!(
            hebun_map.relations[0]
                .relation_type,
            KnowledgeRelationType::
                PossibleContinuation,
        );

        assert!(report.contains_node(
            "papers-hebun-main-md",
        ));
    }

#[test]
    fn unclassified_cluster_is_not_mapped() {
        let clustering_report =
            TopicClusteringReport {
                clusters: vec![
                    TopicCluster {
                        domain:
                            ZanistarastDomain::
                                Unclassified,
                        articles: vec![
                            ClusteredArticle {
                                relative_path:
                                    PathBuf::from(
                                        "notes/general.md",
                                    ),
                                title: None,
                                readiness_score: 10,
                                maturity_level:
                                    ArticleMaturityLevel::
                                        Fragment,
                            },
                        ],
                        total_readiness_score: 10,
                    },
                ],
                total_clustered_articles: 1,
            };

        let report =
            KnowledgeMapBuilder::new().build(
                &clustering_report,
            );

        assert_eq!(report.map_count(), 0);
        assert_eq!(report.total_node_count(), 0);
    }

    #[test]
    fn fragment_relation_requires_review() {
        let nodes = vec![
            sample_knowledge_node(
                "hebun-main",
                "papers/hebun-main.md",
                90,
                ArticleMaturityLevel::StrongCandidate,
            ),
            sample_knowledge_node(
                "hebun-note",
                "notes/hebun-note.md",
                15,
                ArticleMaturityLevel::Fragment,
            ),
        ];

        let relations =
            KnowledgeMapBuilder::build_relations(
                &nodes,
            );

        assert_eq!(relations.len(), 1);

        assert_eq!(
            relations[0].relation_type,
            KnowledgeRelationType::RequiresReview,
        );
    }

    #[test]
    fn knowledge_node_and_relation_validate_required_fields() {
        let valid_node = sample_knowledge_node(
            "hebun-main",
            "papers/hebun-main.md",
            90,
            ArticleMaturityLevel::StrongCandidate,
        );

        assert!(valid_node.is_complete());

        let invalid_node = KnowledgeNode {
            id: String::new(),
            relative_path: PathBuf::new(),
            title: None,
            readiness_score: 0,
            maturity_level:
                ArticleMaturityLevel::Fragment,
        };

        assert!(!invalid_node.is_complete());

        let valid_relation = KnowledgeRelation {
            source_id: "first".to_string(),
            target_id: "second".to_string(),
            relation_type:
                KnowledgeRelationType::SameDomain,
        };

        assert!(valid_relation.is_complete());

        let self_relation = KnowledgeRelation {
            source_id: "same".to_string(),
            target_id: "same".to_string(),
            relation_type:
                KnowledgeRelationType::SameDomain,
        };

        assert!(!self_relation.is_complete());
    }

    #[test]
    fn registers_and_queries_dna_records() {
        let mut dna_map =
            DnaKnowledgeMap::new();

        assert!(dna_map.register(
            DnaKnowledgeRecord::new(
                "hebun-core",
                ZanistarastDnaKind::
                    CorePrinciple,
                "Hebûn değişmez çekirdek ilkedir.",
                true,
            ),
        ));

        assert!(dna_map.register(
            DnaKnowledgeRecord::new(
                "rasterast-concept",
                ZanistarastDnaKind::Concept,
                "Rasterast doğrulama kavramıdır.",
                true,
            ),
        ));

        assert_eq!(dna_map.record_count(), 2);
        assert_eq!(dna_map.records().len(), 2);

        assert_eq!(
            dna_map
                .records_of_kind(
                    ZanistarastDnaKind::Concept,
                )
                .len(),
            1,
        );

        assert!(
            dna_map.contains_node("hebun-core"),
        );

        assert!(
            dna_map
                .remove_record(
                    "rasterast-concept",
                )
                .is_some(),
        );

        assert_eq!(dna_map.record_count(), 1);
    }

#[test]
    fn rejects_invalid_and_duplicate_dna_records() {
        let mut dna_map =
            DnaKnowledgeMap::new();

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
                ZanistarastDnaKind::
                    CorePrinciple,
                "DNA kaydı değişmez olmalıdır.",
                false,
            ),
        ));

        assert!(dna_map.register(
            DnaKnowledgeRecord::new(
                "hebun-core",
                ZanistarastDnaKind::
                    CorePrinciple,
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
    fn registers_and_queries_rna_records() {
        let mut rna_map =
            RnaKnowledgeMap::new();

        assert!(rna_map.register(
            RnaKnowledgeRecord::new(
                "hebun-article-process",
                ZanistarastRnaKind::Process,
                "Hebûn bilgisini makaleye dönüştürür.",
                vec!["hebun-core".to_string()],
                vec!["hebun-paper".to_string()],
            ),
        ));

        assert!(rna_map.register(
            RnaKnowledgeRecord::new(
                "mudebbir-approval",
                ZanistarastRnaKind::
                    ApprovalRequest,
                "Müdebbir onayı ister.",
                vec![
                    "verified-package".to_string(),
                ],
                vec![
                    "approved-package".to_string(),
                ],
            ),
        ));

        assert_eq!(rna_map.record_count(), 2);

        assert_eq!(
            rna_map
                .records_using_source(
                    "hebun-core",
                )
                .len(),
            1,
        );

        assert_eq!(
            rna_map
                .records_producing_target(
                    "approved-package",
                )
                .len(),
            1,
        );

        assert!(
            rna_map
                .records_without_sources()
                .is_empty(),
        );

        assert!(
            rna_map
                .records_without_targets()
                .is_empty(),
        );

        assert!(
            rna_map
                .remove_record(
                    "mudebbir-approval",
                )
                .is_some(),
        );
    }

    #[test]
    fn rejects_invalid_and_duplicate_rna_records() {
        let mut rna_map =
            RnaKnowledgeMap::new();

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
                "invalid-source",
                ZanistarastRnaKind::
                    TransformationRule,
                "Boş kaynak kabul edilmez.",
                vec![" ".to_string()],
                vec!["target".to_string()],
            ),
        ));

        assert!(rna_map.register(
            RnaKnowledgeRecord::new(
                "article-process",
                ZanistarastRnaKind::Process,
                "Makale üretim sürecidir.",
                vec!["plan".to_string()],
                vec!["draft".to_string()],
            ),
        ));

        assert!(!rna_map.register(
            RnaKnowledgeRecord::new(
                "article-process",
                ZanistarastRnaKind::Task,
                "Mükerrer kayıt.",
                Vec::new(),
                Vec::new(),
            ),
        ));
    }

    #[test]
    fn registers_and_queries_protein_records() {
        let mut protein_map =
            ProteinKnowledgeMap::new();

        assert!(protein_map.register(
            ProteinKnowledgeRecord::new(
                "rasterast-paper",
                ZanistarastProteinKind::Article,
                "Rasterast akademik makalesidir.",
                vec![
                    "rasterast-concept".to_string(),
                ],
                Some(PathBuf::from(
                    "papers/rasterast.md",
                )),
                true,
            ),
        ));

        assert!(protein_map.register(
            ProteinKnowledgeRecord::new(
                "verification-report",
                ZanistarastProteinKind::Report,
                "Doğrulama raporudur.",
                vec![
                    "verification-process"
                        .to_string(),
                ],
                Some(PathBuf::from(
                    "reports/verification.md",
                )),
                false,
            ),
        ));

        assert_eq!(
            protein_map.record_count(),
            2,
        );

        assert_eq!(
            protein_map
                .records_of_kind(
                    ZanistarastProteinKind::Article,
                )
                .len(),
            1,
        );

        assert_eq!(
            protein_map.verified_records().len(),
            1,
        );

        assert_eq!(
            protein_map
                .unverified_records()
                .len(),
            1,
        );

        assert!(
            protein_map
                .record_for_node(
                    "rasterast-paper",
                )
                .expect(
                    "article should exist",
                )
                .has_relative_path(),
        );

        assert!(
            protein_map
                .remove_record(
                    "verification-report",
                )
                .is_some(),
        );
    }

#[test]
    fn rejects_invalid_and_duplicate_protein_records() {
        let mut protein_map =
            ProteinKnowledgeMap::new();

        assert!(!protein_map.register(
            ProteinKnowledgeRecord::new(
                "",
                ZanistarastProteinKind::Article,
                "Missing identifier.",
                Vec::new(),
                None,
                false,
            ),
        ));

        assert!(!protein_map.register(
            ProteinKnowledgeRecord::new(
                "invalid-path",
                ZanistarastProteinKind::
                    PublicationPackage,
                "Boş yol kabul edilmez.",
                Vec::new(),
                Some(PathBuf::new()),
                false,
            ),
        ));

        assert!(protein_map.register(
            ProteinKnowledgeRecord::new(
                "hebun-paper",
                ZanistarastProteinKind::Article,
                "Hebûn makalesidir.",
                vec![
                    "hebun-process".to_string(),
                ],
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
                "Mükerrer kayıt.",
                Vec::new(),
                None,
                false,
            ),
        ));
    }

    #[test]
    fn knowledge_layer_map_assigns_and_reports_distribution() {
        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-core",
                ZanistarastKnowledgeLayer::Dna,
                "Çekirdek ilkedir.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-process",
                ZanistarastKnowledgeLayer::Rna,
                "Dönüşüm sürecidir.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-paper",
                ZanistarastKnowledgeLayer::
                    Protein,
                "Somut çıktıdır.",
            ),
        ));

        assert_eq!(
            layer_map.assignment_count(),
            3,
        );

        assert!(layer_map.has_assignment(
            "hebun-core",
        ));

        assert_eq!(
            layer_map.layer_distribution(),
            (1, 1, 1),
        );

        assert_eq!(
            layer_map.assigned_nodes().len(),
            3,
        );

        assert!(!layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-core",
                ZanistarastKnowledgeLayer::
                    Protein,
                "İkinci atama kabul edilmez.",
            ),
        ));
    }

    #[test]
    fn validates_complete_dna_rna_protein_architecture() {
        let mut architecture =
            ZanistarastKnowledgeArchitecture::new();

        assert!(architecture.dna.register(
            DnaKnowledgeRecord::new(
                "hebun-core",
                ZanistarastDnaKind::
                    CorePrinciple,
                "Hebûn değişmez çekirdek ilkedir.",
                true,
            ),
        ));

        assert!(architecture.rna.register(
            RnaKnowledgeRecord::new(
                "hebun-process",
                ZanistarastRnaKind::Process,
                "Hebûn bilgisini çıktıya dönüştürür.",
                vec!["hebun-core".to_string()],
                vec!["hebun-paper".to_string()],
            ),
        ));

        assert!(architecture.protein.register(
            ProteinKnowledgeRecord::new(
                "hebun-paper",
                ZanistarastProteinKind::Article,
                "Hebûn akademik makalesidir.",
                vec![
                    "hebun-core".to_string(),
                    "hebun-process".to_string(),
                ],
                Some(PathBuf::from(
                    "papers/hebun.md",
                )),
                false,
            ),
        ));

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-core",
                ZanistarastKnowledgeLayer::Dna,
                "Çekirdek ilkedir.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-process",
                ZanistarastKnowledgeLayer::Rna,
                "Dönüşüm sürecidir.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-paper",
                ZanistarastKnowledgeLayer::
                    Protein,
                "Somut çıktıdır.",
            ),
        ));

        let validation = architecture
            .validate_layer_alignment(
                &layer_map,
            );

        assert!(validation.is_valid());

        assert_eq!(
            architecture.total_record_count(),
            3,
        );

        assert!(architecture.contains_node(
            "hebun-paper",
        ));

        assert_eq!(
            architecture.layer_for_node(
                "hebun-core",
            ),
            Some(
                ZanistarastKnowledgeLayer::Dna,
            ),
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
    fn reports_invalid_architecture_alignment() {
        let mut architecture =
            ZanistarastKnowledgeArchitecture::new();

        assert!(architecture.dna.register(
            DnaKnowledgeRecord::new(
                "hebun-core",
                ZanistarastDnaKind::
                    CorePrinciple,
                "Hebûn çekirdek ilkedir.",
                true,
            ),
        ));

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-core",
                ZanistarastKnowledgeLayer::
                    Protein,
                "Bilerek yanlış katman.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "missing-process",
                ZanistarastKnowledgeLayer::Rna,
                "Ayrıntılı kaydı yoktur.",
            ),
        ));

        let validation = architecture
            .validate_layer_alignment(
                &layer_map,
            );

        assert!(!validation.is_valid());

        assert_eq!(
            validation.mismatch_count(),
            1,
        );

        assert_eq!(
            validation.missing_count(),
            1,
        );

        assert_eq!(
            validation.mismatched_layer_nodes,
            vec!["hebun-core".to_string()],
        );

        assert_eq!(
            validation.missing_detailed_records,
            vec![
                "missing-process".to_string(),
            ],
        );
    }
#[test]
fn reports_detailed_records_without_layer_assignment() {
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
            "hebun-process",
            ZanistarastRnaKind::Process,
            "Hebûn bilgisini somut çıktıya dönüştürür.",
            vec!["hebun-core".to_string()],
            vec!["hebun-paper".to_string()],
        ),
    ));

    assert!(architecture.protein.register(
        ProteinKnowledgeRecord::new(
            "hebun-paper",
            ZanistarastProteinKind::Article,
            "Hebûn akademik makalesidir.",
            vec![
                "hebun-core".to_string(),
                "hebun-process".to_string(),
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
            "Çekirdek ilkedir.",
        ),
    ));

    let validation =
        architecture.validate_layer_alignment(
            &layer_map,
        );

    assert!(!validation.is_valid());

    assert_eq!(
        validation.unassigned_count(),
        2,
    );

    assert_eq!(
        validation.unassigned_detailed_nodes,
        vec![
            "hebun-paper".to_string(),
            "hebun-process".to_string(),
        ],
    );

    assert!(validation
        .missing_detailed_records
        .is_empty());

    assert!(validation
        .mismatched_layer_nodes
        .is_empty());
}
#[test]
    fn validates_complete_dna_rna_protein_chain() {
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
                "Hebûn çekirdek bilgisini makale çıktısına dönüştürür.",
                vec![
                    "hebun-core".to_string(),
                ],
                vec![
                    "hebun-paper".to_string(),
                ],
            ),
        ));

        assert!(architecture.protein.register(
            ProteinKnowledgeRecord::new(
                "hebun-paper",
                ZanistarastProteinKind::Article,
                "Hebûn hakkındaki somut akademik makaledir.",
                vec![
                    "hebun-core".to_string(),
                    "hebun-article-process"
                        .to_string(),
                ],
                Some(PathBuf::from(
                    "papers/hebun.md",
                )),
                false,
            ),
        ));

        let validation =
            architecture.validate_knowledge_chain();

        assert!(validation.is_valid());

        assert_eq!(
            validation.rna_record_count,
            1,
        );

        assert_eq!(
            validation.protein_record_count,
            1,
        );

        assert_eq!(
            validation.missing_link_count(),
            0,
        );

        assert!(validation
            .missing_rna_source_nodes
            .is_empty());

        assert!(validation
            .missing_rna_target_nodes
            .is_empty());

        assert!(validation
            .missing_protein_source_nodes
            .is_empty());
    }

    #[test]
    fn reports_missing_dna_rna_protein_chain_links() {
        let mut architecture =
            ZanistarastKnowledgeArchitecture::new();

        assert!(architecture.rna.register(
            RnaKnowledgeRecord::new(
                "article-process",
                ZanistarastRnaKind::Process,
                "Eksik DNA kaynağından eksik Protein hedefine ilerler.",
                vec![
                    "missing-dna-source"
                        .to_string(),
                ],
                vec![
                    "missing-protein-target"
                        .to_string(),
                ],
            ),
        ));

        assert!(architecture.protein.register(
            ProteinKnowledgeRecord::new(
                "article-output",
                ZanistarastProteinKind::Article,
                "Kaynak süreci bulunmayan akademik çıktıdır.",
                vec![
                    "missing-rna-process"
                        .to_string(),
                ],
                Some(PathBuf::from(
                    "papers/article.md",
                )),
                false,
            ),
        ));

        let validation =
            architecture.validate_knowledge_chain();

        assert!(!validation.is_valid());

        assert_eq!(
            validation.missing_rna_source_count(),
            1,
        );

        assert_eq!(
            validation.missing_rna_target_count(),
            1,
        );

        assert_eq!(
            validation
                .missing_protein_source_count(),
            1,
        );

        assert_eq!(
            validation.missing_link_count(),
            3,
        );

        assert_eq!(
            validation.missing_rna_source_nodes,
            vec![
                "article-process:missing-dna-source"
                    .to_string(),
            ],
        );

        assert_eq!(
            validation.missing_rna_target_nodes,
            vec![
                "article-process:missing-protein-target"
                    .to_string(),
            ],
        );

        assert_eq!(
            validation
                .missing_protein_source_nodes,
            vec![
                "article-output:missing-rna-process"
                    .to_string(),
            ],
        );
    }

    #[test]
    fn rejects_dna_node_as_rna_target() {
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
                "invalid-process",
                ZanistarastRnaKind::Process,
                "RNA süreci DNA katmanını çıktı olarak üretemez.",
                vec![
                    "hebun-core".to_string(),
                ],
                vec![
                    "hebun-core".to_string(),
                ],
            ),
        ));

        let validation =
            architecture.validate_knowledge_chain();

        assert!(!validation.is_valid());

        assert!(validation
            .missing_rna_source_nodes
            .is_empty());

        assert_eq!(
            validation.missing_rna_target_nodes,
            vec![
                "invalid-process:hebun-core"
                    .to_string(),
            ],
        );

        assert_eq!(
            validation.missing_link_count(),
            1,
        );
    }
#[test]
    fn validates_complete_knowledge_architecture_health() {
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
                "hebun-process",
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
                    "hebun-process".to_string(),
                ],
                Some(PathBuf::from(
                    "papers/hebun.md",
                )),
                false,
            ),
        ));

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-core",
                ZanistarastKnowledgeLayer::Dna,
                "Değişmez çekirdek ilkedir.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-process",
                ZanistarastKnowledgeLayer::Rna,
                "Bilgi dönüşüm sürecidir.",
            ),
        ));

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-paper",
                ZanistarastKnowledgeLayer::Protein,
                "Somut akademik çıktıdır.",
            ),
        ));

        let health =
            architecture.validate_architecture(
                &layer_map,
            );

        assert!(health.is_valid());
        assert!(!health.has_issues());

        assert_eq!(
            health.layer_issue_count(),
            0,
        );

        assert_eq!(
            health.chain_issue_count(),
            0,
        );

        assert_eq!(
            health.total_issue_count(),
            0,
        );
    }

    #[test]
    fn reports_combined_knowledge_architecture_issues() {
        let mut architecture =
            ZanistarastKnowledgeArchitecture::new();

        assert!(architecture.rna.register(
            RnaKnowledgeRecord::new(
                "article-process",
                ZanistarastRnaKind::Process,
                "Eksik kaynaktan eksik hedefe ilerler.",
                vec![
                    "missing-dna-source".to_string(),
                ],
                vec![
                    "missing-protein-target"
                        .to_string(),
                ],
            ),
        ));

        let layer_map =
            KnowledgeLayerMap::new();

        let health =
            architecture.validate_architecture(
                &layer_map,
            );

        assert!(!health.is_valid());
        assert!(health.has_issues());

        assert_eq!(
            health.layer_issue_count(),
            1,
        );

        assert_eq!(
            health.chain_issue_count(),
            2,
        );

        assert_eq!(
            health.total_issue_count(),
            3,
        );

        assert_eq!(
            health
                .layer_alignment
                .unassigned_detailed_nodes,
            vec!["article-process".to_string()],
        );

        assert_eq!(
            health
                .knowledge_chain
                .missing_rna_source_nodes,
            vec![
                "article-process:missing-dna-source"
                    .to_string(),
            ],
        );

        assert_eq!(
            health
                .knowledge_chain
                .missing_rna_target_nodes,
            vec![
                "article-process:missing-protein-target"
                    .to_string(),
            ],
        );
    }

}



       
   
