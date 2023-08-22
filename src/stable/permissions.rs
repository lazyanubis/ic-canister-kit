use std::collections::{HashMap, HashSet};

use crate::identity::UserId;

use super::Stable;

/// 权限管理

// 单个权限对象
#[derive(Debug, Default)]
pub struct Permission {
    pub permission: String,     // 权限名称
    pub users: HashSet<UserId>, // 拥有该权限的用户
}

// 单个权限对象持久化
pub type PermissionState = (String, Vec<UserId>);

impl Stable<PermissionState, PermissionState> for Permission {
    fn store(&mut self) -> PermissionState {
        let permission = std::mem::take(&mut self.permission);
        let users = std::mem::take(&mut self.users).into_iter().collect();
        (permission, users)
    }

    fn restore(&mut self, state: PermissionState) {
        let _ = std::mem::replace(&mut self.permission, state.0);
        let _ = std::mem::replace(&mut self.users, state.1.into_iter().collect());
    }
}

impl From<PermissionState> for Permission {
    fn from(value: PermissionState) -> Self {
        Permission {
            permission: value.0,
            users: value.1.into_iter().collect(),
        }
    }
}

// 多个权限对象
#[derive(Debug, Default)]
pub struct Permissions {
    pub permissions: Vec<Permission>,            // 所有权限记录
    pub permissions_map: HashMap<String, usize>, // 快速查询
}

// 多个权限对象持久化
pub type PermissionsState = (Vec<PermissionState>,);

impl Stable<PermissionsState, PermissionsState> for Permissions {
    fn store(&mut self) -> PermissionsState {
        ((&mut self.permissions)
            .into_iter()
            .map(|s| s.store())
            .collect::<Vec<PermissionState>>(),)
    }

    fn restore(&mut self, state: PermissionsState) {
        let permissions = state.0.into_iter().map(|s| s.into()).collect();
        let _ = std::mem::replace(&mut self.permissions, permissions);
        self.permissions_map = {
            let mut map = HashMap::with_capacity(self.permissions.len());
            for (i, v) in self.permissions.iter().enumerate() {
                map.insert(v.permission.clone(), i);
            }
            map
        };
    }
}

impl Permissions {
    // 初始化某权限数据
    fn assure_permission(&mut self, permission: &String) {
        if !self.permissions_map.contains_key(permission) {
            // 不存在该权限则初始化
            self.permissions.push(Permission {
                permission: permission.to_string(),
                users: HashSet::new(),
            });
            // 记录顺序
            self.permissions_map
                .insert(permission.to_string(), self.permissions.len() - 1);
        }
    }
    // 插入权限
    pub fn insert(&mut self, permission: &str, user_id: UserId) {
        let permission = permission.to_string();
        self.assure_permission(&permission); // 确保有这个权限名称

        let permission = &mut self.permissions[self.permissions_map[&permission]];
        if !permission.users.contains(&user_id) {
            // 不存在则加入
            permission.users.insert(user_id);
        }
    }
    // 移除权限
    pub fn remove(&mut self, permission: &str, user_id: &UserId) {
        let index = self.permissions_map.get(permission);
        if index.is_none() {
            return;
        }
        let index = index.unwrap();
        let permission = &mut self.permissions[*index];
        permission.users.remove(&user_id); // 移除
    }

    // 判断是否有某权限
    pub fn has_permission(&self, permission: &str, user_id: UserId) -> bool {
        let index = self.permissions_map.get(permission);
        if index.is_none() {
            return false;
        }
        let index = index.unwrap();
        let permission = &self.permissions[*index];
        permission.users.contains(&user_id)
    }

    // 获取某权限的所有用户
    pub fn users<'a>(&'a self, permission: &str) -> Option<&'a HashSet<UserId>> {
        let index = self.permissions_map.get(permission);
        if index.is_none() {
            return None;
        }
        let index = index.unwrap();
        let permission = &self.permissions[*index];
        Some(&permission.users)
    }
}
