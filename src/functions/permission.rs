use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::identity::UserId;

/// 权限管理

// 被管理的用户类型
#[derive(candid::CandidType, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

/// 权限修改参数
#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub enum PermissionUpdatedArg {
    /// 更新用户权限
    UpdateUserPermission(UserId, Option<HashSet<Permission>>),
    /// 更新角色权限
    UpdateRolePermission(String, Option<HashSet<Permission>>),
    /// 更新用户角色
    UpdateUserRole(UserId, Option<HashSet<String>>),
}

/// 权限更新错误
#[derive(Debug)]
pub enum PermissionUpdatedError {
    /// 权限不存在错误
    InvalidPermission(Permission),
    /// 角色不存在错误
    InvalidRole(String),
}
impl Display for PermissionUpdatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionUpdatedError::InvalidPermission(permission) => {
                write!(f, "InvalidPermission({permission:?})")
            }
            PermissionUpdatedError::InvalidRole(role) => write!(f, "InvalidRole({role})"),
        }
    }
}
impl std::error::Error for PermissionUpdatedError {}

/// 权限管理
pub trait Permissable {
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
        args: Vec<PermissionUpdatedArg>,
    ) -> Result<(), PermissionUpdatedError>;
}
