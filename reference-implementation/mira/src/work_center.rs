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
/// Müdebbirden veya kullanıcıdan bilgi girişi bekleyen işleri döndürür.
    pub fn items_awaiting_input(&self) -> Vec<&WorkItem> {
        self.items_with_status(WorkItemStatus::AwaitingInput)
    }
/// Rasterast veya başka bir doğrulama katmanını bekleyen işleri döndürür.
    pub fn items_awaiting_verification(&self) -> Vec<&WorkItem> {
        self.items_with_status(WorkItemStatus::AwaitingVerification)
    }

    /// Henüz tamamlanmamış çalışma merkezi işlerini döndürür.
    pub fn active_items(&self) -> Vec<&WorkItem> {
        self.items
            .iter()
            .filter(|item| item.status != WorkItemStatus::Ready)
            .collect()
    }
 /// Mira'nın doğrudan ele alabileceği ilk hazır işi döndürür.
    pub fn next_ready_item(&self) -> Option<&WorkItem> {
        self.items
            .iter()
            .find(|item| item.status == WorkItemStatus::Ready)
    }
/// İlerlemesi bir engel nedeniyle durmuş işleri döndürür.
    pub fn blocked_items(&self) -> Vec<&WorkItem> {
        self.items_with_status(WorkItemStatus::Blocked)
    }
/// Çalışma merkezinin kısa durum özetini döndürür.
    pub fn summary(&self) -> String {
        format!(
            "total={}, active={}, ready={}, blocked={}, awaiting_input={}, awaiting_approval={}, awaiting_verification={}",
            self.item_count(),
            self.active_items().len(),
            self.items_with_status(WorkItemStatus::Ready).len(),
            self.blocked_items().len(),
            self.items_awaiting_input().len(),
            self.items_requiring_review().len(),
            self.items_awaiting_verification().len(),
        )
    }
/// Mira'nın bugün ele alması gereken en uygun işi döndürür.
    ///
    /// Öncelik sırası:
    /// 1. Hazır iş
    /// 2. Girdi bekleyen iş
    /// 3. Onay bekleyen iş
    /// 4. Doğrulama bekleyen iş
    /// 5. Engellenmiş iş
    pub fn what_should_we_do_today(&self) -> Option<&WorkItem> {
        self.next_ready_item()
            .or_else(|| {
                self.items_awaiting_input()
                    .into_iter()
                    .next()
            })
            .or_else(|| {
                self.items_requiring_review()
                    .into_iter()
                    .next()
            })
            .or_else(|| {
                self.items_awaiting_verification()
                    .into_iter()
                    .next()
            })
            .or_else(|| {
                self.blocked_items()
                    .into_iter()
                    .next()
            })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_center_reports_and_selects_active_work() {
        let mut center = WorkCenter::default();

        assert!(center.add_item(WorkItem::new(
            "ready-paper",
            "Hazır makaleyi işle",
            WorkItemStatus::Ready,
        )));

        assert!(center.add_item(WorkItem::new(
            "approval-required",
            "Müdebbir onayını bekle",
            WorkItemStatus::AwaitingApproval,
        )));

        assert!(center.add_item(WorkItem::new(
            "verification-required",
            "Rasterast doğrulamasını bekle",
            WorkItemStatus::AwaitingVerification,
        )));

        assert!(center.add_item(WorkItem::new(
            "blocked-task",
            "Engellenmiş görevi incele",
            WorkItemStatus::Blocked,
        )));

        assert_eq!(center.item_count(), 4);

        assert_eq!(
            center.items_requiring_review().len(),
            1,
        );

        assert_eq!(
            center.items_awaiting_verification().len(),
            1,
        );

        assert_eq!(
            center.blocked_items().len(),
            1,
        );

        let next = center
            .what_should_we_do_today()
            .expect("ready work item should exist");

        assert_eq!(next.id, "ready-paper");
        assert_eq!(next.status, WorkItemStatus::Ready);
    }
}




