use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{common::option::display_option_by, identity::UserId};

// 权限管理

/// 权限修改参数
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum PermissionUpdatedArg<Permission: Eq + Hash> {
    /// 更新用户权限
    UpdateUserPermission(UserId, Option<HashSet<Permission>>),
    /// 更新角色权限
    UpdateRolePermission(String, Option<HashSet<Permission>>),
    /// 更新用户角色
    UpdateUserRole(UserId, Option<HashSet<String>>),
}

/// 权限更新错误
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum PermissionUpdatedError<Permission> {
    /// 权限不存在错误
    InvalidPermission(Permission),
    /// 角色不存在错误
    InvalidRole(String),
}
impl<Permission: Debug> Display for PermissionUpdatedError<Permission> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionUpdatedError::InvalidPermission(permission) => {
                write!(f, "InvalidPermission({permission:?})")
            }
            PermissionUpdatedError::InvalidRole(role) => write!(f, "InvalidRole({role})"),
        }
    }
}
impl<Permission: Debug> std::error::Error for PermissionUpdatedError<Permission> {}

/// 权限管理
pub trait Permissable<Permission: Eq + Hash> {
    // 查询
    ///  当前管理的所有用户 包括直接授权的和通过角色授权的
    fn permission_users(&self) -> HashSet<&UserId>;
    ///  当前管理的所有角色
    fn permission_roles(&self) -> HashSet<&String>;

    ///  某用户被直接授权的权限
    fn permission_assigned(&self, user_id: &UserId) -> Option<&HashSet<Permission>>;
    ///  某角色被直接授权的权限
    fn permission_role_assigned(&self, role: &str) -> Option<&HashSet<Permission>>;
    /// 某用户被授权的角色
    fn permission_user_roles(&self, user_id: &UserId) -> Option<&HashSet<String>>;

    // 综合直接授权和角色间接授权的情况
    // 若权限是默认没有的(Permitted)，任意路径包含则有该权限，fallback 是 无
    // 若权限是默认拥有的(Forbidden)，任意路径包含则无该权限，fallback 是 有

    /// 判断用户是否拥有某权限
    fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool;
    /// 获取用户的综合权限情况
    fn permission_owned(&self, user_id: &UserId) -> HashMap<&Permission, bool>;

    // 修改

    /// 重置管理的权限，防止版本更新导致某一个权限的信息不一致
    fn permission_reset(&mut self, permissions: HashSet<Permission>);
    /// 权限更新
    fn permission_update(
        &mut self,
        args: Vec<PermissionUpdatedArg<Permission>>,
    ) -> Result<(), PermissionUpdatedError<Permission>>;
}

impl Display for PermissionUpdatedArg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UpdateUserPermission(user_id, permissions) => f.write_str(&format!(
                "update user: {} permissions: {}",
                user_id.to_text(),
                display_option_by(permissions, |permissions| format!(
                    "[{}]",
                    permissions
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
            )),
            Self::UpdateRolePermission(role, permissions) => f.write_str(&format!(
                "update role: {} permissions: {}",
                role,
                display_option_by(permissions, |permissions| format!(
                    "[{}]",
                    permissions
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
            )),
            Self::UpdateUserRole(user_id, roles) => f.write_str(&format!(
                "update user: {} roles: {}",
                user_id.to_text(),
                display_option_by(roles, |roles| format!(
                    "[{}]",
                    roles.iter().cloned().collect::<Vec<_>>().join(",")
                ))
            )),
        }
    }
}

// ================== 简单实现 ==================

/// 权限功能简单实现
pub mod basic {
    use std::{
        collections::{HashMap, HashSet},
        fmt::Display,
    };

    use candid::CandidType;
    use serde::{Deserialize, Serialize};

    use crate::{
        functions::types::{Permissable, PermissionUpdatedArg, PermissionUpdatedError},
        identity::UserId,
    };

    /// 被管理的用户类型
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Permission {
        /// 授权类型 默认没有该权限 只有被加入的用户才有该权限
        Permitted(String),
        /// 禁止类型 默认拥有该权限 如果被加入了就没有该权限了
        Forbidden(String),
    }

    impl Display for Permission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Permission::Permitted(name) => write!(f, "Permitted({name})"),
                Permission::Forbidden(name) => write!(f, "Forbidden({name})"),
            }
        }
    }

    impl Permission {
        /// 构造许可权限
        pub fn by_permit(name: &str) -> Self {
            Permission::Permitted(name.to_string())
        }
        /// 构造禁止权限
        pub fn by_forbid(name: &str) -> Self {
            Permission::Forbidden(name.to_string())
        }
        /// 判断是否许可权限
        pub fn is_permit(&self) -> bool {
            matches!(self, Self::Permitted(_))
        }
        /// 判断是否禁止权限
        pub fn is_forbid(&self) -> bool {
            matches!(self, Self::Forbidden(_))
        }
        /// 文本化
        pub fn name(&self) -> &str {
            match self {
                Permission::Permitted(name) => name,
                Permission::Forbidden(name) => name,
            }
        }
    }

    /// 多个权限对象
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
    pub struct Permissions {
        /// 所有权限种类
        pub permissions: HashSet<Permission>,
        /// 用户分配的特别权限, Permitted表示拥有, Forbidden表示禁止
        pub user_permissions: HashMap<UserId, HashSet<Permission>>,
        /// 某角色对权限的限制
        pub role_permissions: HashMap<String, HashSet<Permission>>,
        /// 用户被授权的角色
        pub user_roles: HashMap<UserId, HashSet<String>>,
    }

    impl Permissions {
        // 检查一定存在权限
        fn assure_permission_exist(
            &self,
            permissions: &Option<HashSet<Permission>>,
        ) -> Result<(), PermissionUpdatedError<Permission>> {
            if let Some(permissions) = permissions {
                for permission in permissions {
                    if !self.permissions.contains(permission) {
                        return Err(PermissionUpdatedError::InvalidPermission(
                            permission.clone(),
                        ));
                    }
                }
            }

            Ok(())
        }
        // 检查一定存在角色
        fn assure_role_exist(
            &self,
            roles: &Option<HashSet<String>>,
        ) -> Result<(), PermissionUpdatedError<Permission>> {
            if let Some(roles) = roles {
                for role in roles {
                    if !self.role_permissions.contains_key(role) {
                        return Err(PermissionUpdatedError::InvalidRole(role.clone()));
                    }
                }
            }
            Ok(())
        }
    }

    impl Permissable<Permission> for Permissions {
        // 查询
        fn permission_users(&self) -> HashSet<&UserId> {
            let mut users: HashSet<&UserId> = self.user_roles.keys().collect();
            users.extend(self.user_permissions.keys());
            users
        }
        fn permission_roles(&self) -> HashSet<&String> {
            self.role_permissions.keys().collect()
        }

        fn permission_assigned(&self, user_id: &UserId) -> Option<&HashSet<Permission>> {
            self.user_permissions.get(user_id)
        }
        fn permission_role_assigned(&self, role: &str) -> Option<&HashSet<Permission>> {
            self.role_permissions.get(role)
        }
        fn permission_user_roles(&self, user_id: &UserId) -> Option<&HashSet<String>> {
            self.user_roles.get(user_id)
        }

        fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool {
            // 单独指定
            if let Some(permissions) = self.user_permissions.get(user_id) {
                if permissions.contains(permission) {
                    return match permission {
                        Permission::Permitted(_) => true,
                        Permission::Forbidden(_) => false,
                    };
                }
            }
            // 角色自定
            if let Some(roles) = self.user_roles.get(user_id) {
                for role in roles {
                    if let Some(permissions) = self.role_permissions.get(role) {
                        if permissions.contains(permission) {
                            return match permission {
                                Permission::Permitted(_) => true,
                                Permission::Forbidden(_) => false,
                            };
                        }
                    }
                }
            }
            // 不存在则默认
            match permission {
                Permission::Permitted(_) => false,
                Permission::Forbidden(_) => true,
            }
        }
        fn permission_owned(&self, user_id: &UserId) -> HashMap<&Permission, bool> {
            self.permissions
                .iter()
                .map(|permission| (permission, self.permission_has(user_id, permission)))
                .collect()
        }

        // 修改
        fn permission_reset(&mut self, permissions: HashSet<Permission>) {
            self.permissions = permissions;
            // 核对其他数据中的权限是否正确
            self.role_permissions
                .iter_mut()
                .for_each(|(_, permissions)| {
                    let mut removed = Vec::new();
                    for permission in permissions.iter() {
                        if !self.permissions.contains(permission) {
                            removed.push(permission.clone());
                        }
                    }
                    for permission in removed {
                        permissions.remove(&permission);
                    }
                });
            self.user_permissions
                .iter_mut()
                .for_each(|(_, permissions)| {
                    let mut removed = Vec::new();
                    for permission in permissions.iter() {
                        if !self.permissions.contains(permission) {
                            removed.push(permission.clone());
                        }
                    }
                    for permission in removed {
                        permissions.remove(&permission);
                    }
                });
        }
        fn permission_update(
            &mut self,
            args: Vec<PermissionUpdatedArg<Permission>>,
        ) -> Result<(), PermissionUpdatedError<Permission>> {
            for arg in args.iter() {
                match arg {
                    PermissionUpdatedArg::UpdateUserPermission(user_id, permissions) => {
                        // 先检查权限是否都存在
                        self.assure_permission_exist(permissions)?;

                        let exist = self.user_permissions.get(user_id);
                        if let Some(permissions) = &permissions {
                            if let Some(exist) = exist {
                                if exist == permissions {
                                    continue;
                                }
                            }
                        } else if exist.is_none() {
                            continue;
                        }
                        if let Some(permissions) = permissions {
                            self.user_permissions.insert(*user_id, permissions.clone());
                        } else {
                            self.user_permissions.remove(user_id);
                        }
                    }
                    PermissionUpdatedArg::UpdateRolePermission(role, permissions) => {
                        // 先检查权限是否都存在
                        self.assure_permission_exist(permissions)?;

                        let exist = self.role_permissions.get(role);
                        if let Some(permissions) = permissions {
                            if let Some(exist) = exist {
                                if exist == permissions {
                                    continue;
                                }
                            }
                        } else if exist.is_none() {
                            continue;
                        }
                        if let Some(permissions) = permissions {
                            self.role_permissions
                                .insert(role.clone(), permissions.clone());
                        } else {
                            self.role_permissions.remove(role);
                            // 移除要检查用户角色数据对不对
                            self.user_roles.iter_mut().for_each(|(_, roles)| {
                                let mut removed = Vec::new();
                                for role in roles.iter() {
                                    if !self.role_permissions.contains_key(role) {
                                        removed.push(role.clone());
                                    }
                                }
                                for role in removed {
                                    roles.remove(&role);
                                }
                            });
                        }
                    }
                    PermissionUpdatedArg::UpdateUserRole(user_id, roles) => {
                        // 先检查角色是否都存在
                        self.assure_role_exist(roles)?;

                        let exist = self.user_roles.get(user_id);
                        if let Some(roles) = &roles {
                            if let Some(exist) = exist {
                                if exist == roles {
                                    continue;
                                }
                            }
                        } else if exist.is_none() {
                            continue;
                        }
                        if let Some(roles) = roles {
                            self.user_roles.insert(*user_id, roles.clone());
                        } else {
                            self.user_roles.remove(user_id);
                        }
                    }
                }
            }
            Ok(())
        }
    }

    impl PermissionUpdatedArg<String> {
        /// 解析权限，返回 PermissionUpdatedArg
        ///
        /// # Arguments
        ///
        /// * `f` - 权限解析函数，将字符串解析为 Permission
        pub fn parse_permission<E, F: Fn(&str) -> Result<Permission, E>>(
            self,
            f: F,
        ) -> Result<PermissionUpdatedArg<Permission>, E> {
            Ok(match self {
                PermissionUpdatedArg::UpdateUserPermission(user_id, permissions) => {
                    PermissionUpdatedArg::UpdateUserPermission(
                        user_id,
                        permissions
                            .map(|ps| {
                                ps.into_iter()
                                    .map(|p| f(&p))
                                    .collect::<Result<HashSet<_>, _>>()
                            })
                            .transpose()?,
                    )
                }
                PermissionUpdatedArg::UpdateRolePermission(role, permissions) => {
                    PermissionUpdatedArg::UpdateRolePermission(
                        role,
                        permissions
                            .map(|ps| {
                                ps.into_iter()
                                    .map(|p| f(&p))
                                    .collect::<Result<HashSet<_>, _>>()
                            })
                            .transpose()?,
                    )
                }
                PermissionUpdatedArg::UpdateUserRole(user_id, roles) => {
                    PermissionUpdatedArg::UpdateUserRole(user_id, roles)
                }
            })
        }
    }

    // ================= 工具方法 =================

    /// 解析所有权限
    pub fn parse_all_permissions<'a, F, E>(
        actions: &[&'a str],
        parse: F,
    ) -> Result<Vec<Permission>, E>
    where
        F: Fn(&'a str) -> Result<Permission, E>,
    {
        let mut permissions = Vec::with_capacity(actions.len());
        for name in actions {
            permissions.push(parse(name)?);
        }
        Ok(permissions)
    }

    /// 超级管理员获取所有授权权限
    pub fn permitted_permissions(permissions: &HashSet<Permission>) -> HashSet<Permission> {
        permissions
            .iter()
            .filter(|p| p.is_permit())
            .cloned()
            .collect()
    }

    /// 超级管理员获取所有权限
    pub fn supers_updated(
        supers: &[UserId],
        permissions: &HashSet<Permission>,
    ) -> Vec<PermissionUpdatedArg<Permission>> {
        let permitted: HashSet<Permission> = permitted_permissions(permissions);
        supers
            .iter()
            .map(|su| PermissionUpdatedArg::UpdateUserPermission(*su, Some(permitted.clone())))
            .collect()
    }
}
