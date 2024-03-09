use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{common::option::display_option_by, identity::UserId};

/// 权限管理

/// 权限修改参数
#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub enum PermissionUpdatedArg<Permission: Eq + Hash> {
    /// 更新用户权限
    UpdateUserPermission(UserId, Option<HashSet<Permission>>),
    /// 更新角色权限
    UpdateRolePermission(String, Option<HashSet<Permission>>),
    /// 更新用户角色
    UpdateUserRole(UserId, Option<HashSet<String>>),
}

/// 权限更新错误
#[derive(Debug)]
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

    /// 综合直接授权和角色间接授权的情况
    /// 若权限是默认没有的(Permitted)，任意路径包含则有该权限，fallback 是 无
    /// 若权限是默认拥有的(Forbidden)，任意路径包含则无该权限，fallback 是 有

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
