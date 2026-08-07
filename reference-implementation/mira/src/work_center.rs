use serde::{Deserialize, Serialize};

/// Mira aktif çalışma merkezindeki bir işin durumunu belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum WorkItemStatus {
    Pending,
    InProgress,
    Blocked,
    AwaitingInput,
    AwaitingApproval,
    AwaitingVerification,
    Ready,
}

/// Mira aktif çalışma merkezindeki tek bir işi temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct WorkItem {
    pub id: String,
    pub title: String,
    pub status: WorkItemStatus,
}

impl WorkItem {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        status: WorkItemStatus,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            status,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.title.trim().is_empty()
    }
}

/// Mira'nın görünür aktif çalışma merkezidir.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct WorkCenter {
    pub items: Vec<WorkItem>,
}

impl WorkCenter {
    /// Geçerli bir işi çalışma merkezine ekler.
    /// Geçersiz işler eklenmez.
    pub fn add_item(&mut self, item: WorkItem) -> bool {
        if !item.is_valid() {
            return false;
        }

        self.items.push(item);
        true
    }
/// Çalışma merkezindeki toplam iş sayısını döndürür.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Belirtilen durumdaki işleri döndürür.
    pub fn items_with_status(
        &self,
        status: WorkItemStatus,
    ) -> Vec<&WorkItem> {
        self.items
            .iter()
            .filter(|item| item.status == status)
            .collect()
    }
/// Müdebbir incelemesi veya kararı bekleyen işleri döndürür.
    pub fn items_requiring_review(&self) -> Vec<&WorkItem> {
    self.items_with_status(WorkItemStatus::AwaitingApproval)
}

}


