use crate::{MiraCore, MiraTaskStatus};

impl MiraCore {
    /// Görevi planlama aşamasına geçirir.
    pub fn plan_task(&mut self, task_id: uuid::Uuid) -> bool {
        if let Some(task) = self.find_task_mut(task_id) {
            task.update_status(MiraTaskStatus::Planning);
            return true;
        }
        false
    }

    /// Görevi çalıştırma kuyruğuna alır.
    pub fn start_task(&mut self, task_id: uuid::Uuid) -> bool {
        if let Some(task) = self.find_task_mut(task_id) {
            task.update_status(MiraTaskStatus::Running);
            return true;
        }
        false
    }

    /// Görevi Rasterast doğrulamasına gönderir.
    pub fn send_to_rasterast(&mut self, task_id: uuid::Uuid) -> bool {
        if let Some(task) = self.find_task_mut(task_id) {
            task.update_status(MiraTaskStatus::AwaitingRasterast);
            return true;
        }
        false
    }

    /// Görevi Müdebbir onayına gönderir.
    pub fn request_mudebbir_approval(&mut self, task_id: uuid::Uuid) -> bool {
        if let Some(task) = self.find_task_mut(task_id) {
            task.update_status(MiraTaskStatus::AwaitingMudebbir);
            return true;
        }
        false
    }

    /// Görevi tamamlar.
    pub fn complete_task(&mut self, task_id: uuid::Uuid) -> bool {
        if let Some(task) = self.find_task_mut(task_id) {
            task.update_status(MiraTaskStatus::Completed);
            return true;
        }
        false
    }
}



