use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::model::notification::Notification;

// Singleton Database untuk menyimpan notifikasi sementara
lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {
}
