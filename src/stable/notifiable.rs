use std::{collections::HashSet, fmt::Display};

use crate::{
    common::pages::{page_find_with_reserve, page_find_with_reserve_and_filter, Page, PageData},
    identity::CallerId,
    times::TimestampNanos,
};

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub enum NotificationAction {
    Silence,       // 仅仅记录, 不通知
    Email(String), // 邮箱通知, 里面是邮箱地址
    Lark(String),  // Lark 通知, 里面是请求接口
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub created: TimestampNanos,
    pub caller: CallerId,
    pub action: NotificationAction,
    pub title: String,
    pub content: String,
    pub read: TimestampNanos, // 0 表示未读/未通知 是否已读/已通知
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct NotificationSearch {
    pub id: Option<(Option<u64>, Option<u64>)>, // id 过滤
    pub created: Option<(Option<TimestampNanos>, Option<TimestampNanos>)>, // 创建时间过滤
    pub caller: Option<HashSet<CallerId>>,      // 生产人过滤
    pub title: Option<String>,                  // 通知标题过滤
    pub read: Option<bool>,                     // 是否已读过滤
}

impl NotificationSearch {
    fn test(&self, notification: &Notification) -> bool {
        if let Some(id) = self.id {
            let (id_min, id_max) = id;
            if let Some(id_min) = id_min {
                if notification.id < id_min {
                    return false;
                }
            }
            if let Some(id_max) = id_max {
                if id_max < notification.id {
                    return false;
                }
            }
        }
        if let Some(created) = self.created {
            let (created_min, created_max) = created;
            if let Some(created_min) = created_min {
                if notification.created < created_min {
                    return false;
                }
            }
            if let Some(created_max) = created_max {
                if created_max < notification.created {
                    return false;
                }
            }
        }
        if let Some(caller) = &self.caller {
            if !caller.contains(&notification.caller) {
                return false;
            }
        }
        if let Some(title) = &self.title {
            if !notification.title.contains(title) {
                return false;
            }
        }
        if let Some(read) = &self.read {
            if *read && notification.read == 0 {
                return false;
            }
            if !read && notification.read != 0 {
                return false;
            }
        }
        true
    }
}

pub trait Notifiable {
    // 查询
    fn notification_find_all(&self, search: &Option<NotificationSearch>) -> Vec<&Notification>;
    fn notification_find_by_page(
        &self,
        search: &Option<NotificationSearch>,
        page: &Page,
        max: u32,
    ) -> PageData<&Notification>;
    // 修改
    fn notification_push(
        &mut self,
        caller: CallerId,
        action: NotificationAction,
        title: String,
        content: String,
    ) -> u64;
    fn notification_read(&mut self, notification_id: u64);
    fn notification_remove(&mut self, notification_id: u64) -> Option<Notification>;
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone, Default)]
pub struct Notifications {
    pub next_id: u64,
    pub notifications: Vec<Notification>,
}

impl Notifiable for Notifications {
    // 查询
    fn notification_find_all(&self, search: &Option<NotificationSearch>) -> Vec<&Notification> {
        if let Some(search) = search {
            let notifications: Vec<&Notification> = self
                .notifications
                .iter()
                .filter(|record| search.test(record))
                .collect();
            return notifications;
        }
        self.notifications.iter().collect()
    }

    fn notification_find_by_page(
        &self,
        search: &Option<NotificationSearch>,
        page: &Page,
        max: u32,
    ) -> PageData<&Notification> {
        if let Some(search) = search {
            return page_find_with_reserve_and_filter(
                &self.notifications,
                page,
                max,
                |notification| search.test(notification),
            );
        }
        page_find_with_reserve(&self.notifications, page, max)
    }

    // 修改
    fn notification_push(
        &mut self,
        caller: CallerId,
        action: NotificationAction,
        title: String,
        content: String,
    ) -> u64 {
        let id = self.next_id;

        self.next_id += 1;

        self.notifications.push(Notification {
            id,
            created: crate::times::now(),
            caller,
            action,
            title,
            content,
            read: 0,
        });

        id
    }

    fn notification_read(&mut self, notification_id: u64) {
        let now = crate::times::now();
        let mut i = self.notifications.len();
        while 0 < i {
            let notification = &mut self.notifications[i - 1];
            if notification.id == notification_id {
                notification.read = now;
                return;
            }
            i -= 1;
        }
    }

    fn notification_remove(&mut self, notification_id: u64) -> Option<Notification> {
        let mut i = self.notifications.len();
        while 0 < i {
            let notification = &mut self.notifications[i - 1];
            if notification.id == notification_id {
                return Some(self.notifications.remove(i - 1));
            }
            i -= 1;
        }
        None
    }
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{{ id: {}, created: {}, caller: {}, action: {:?}, title: {}, content: {}, read: {} }}",
            self.id,
            self.created,
            self.caller.to_text(),
            self.action,
            self.title,
            self.content,
            self.read,
        ))
    }
}
