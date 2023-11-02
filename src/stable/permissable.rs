use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::{
    canister::call::call_canister,
    identity::{caller, CanisterId, UserId},
    times::schedulable::async_execute,
};

use super::recordable::{format_option, format_option_with_func};

/// 权限管理

// 被管理的用户类型
#[derive(candid::CandidType, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    // 授权类型 默认没有该权限 只有被加入的用户才有该权限
    Permitted(String),
    // 禁止类型 默认拥有该权限 如果被加入了就没有该权限了
    Forbidden(String),
}

impl Permission {
    pub fn is_permit(&self) -> bool {
        matches!(self, Self::Permitted(_))
    }
    pub fn is_forbid(&self) -> bool {
        matches!(self, Self::Forbidden(_))
    }
    pub fn by_permit(name: &str) -> Self {
        Permission::Permitted(name.to_string())
    }
    pub fn by_forbid(name: &str) -> Self {
        Permission::Forbidden(name.to_string())
    }
    pub fn name<'a>(&'a self) -> &'a str {
        match self {
            Permission::Permitted(name) => name,
            Permission::Forbidden(name) => name,
        }
    }
}

// 多个权限对象
#[derive(candid::CandidType, serde::Deserialize, Debug, Default)]
pub struct Permissions {
    pub host: Option<CanisterId>, // 本数据有宿主, 一旦变更调用宿主的业务接口
    pub listener: Option<CanisterId>, // 本数据有监听者, 一旦变更调用监听者的通用接口
    pub permissions: HashSet<Permission>, // 所有权限
    pub user_permissions: HashMap<UserId, HashSet<Permission>>, // 用户分配的特别权限, Permitted表示拥有, Forbidden表示禁止
    pub role_permissions: HashMap<String, HashSet<Permission>>, // 某角色对权限的限制
    pub user_roles: HashMap<UserId, HashSet<String>>,           // 用户所拥有的角色
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub enum PermissionUpdatedArg<T: candid::CandidType + Eq + Hash> {
    UpdateUserPermission(UserId, Option<HashSet<T>>),
    UpdateRolePermission(String, Option<HashSet<T>>),
    UpdateUserRole(UserId, Option<HashSet<String>>),
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct PermissionReplacedArg<T: candid::CandidType + Eq + Hash> {
    pub permissions: HashSet<T>,
    pub user_permissions: HashMap<UserId, HashSet<T>>,
    pub role_permissions: HashMap<String, HashSet<T>>,
    pub user_roles: HashMap<UserId, HashSet<String>>,
}

pub trait Permissable {
    // 查询
    fn permission_host_find(&self) -> Option<CanisterId>;
    fn permission_users(&self) -> HashSet<UserId>;
    fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool;
    fn permission_owned(&self, user_id: &UserId) -> HashMap<&Permission, bool>;
    // 修改
    fn permission_host_update(&mut self, host: Option<CanisterId>, notice: bool);
    fn permission_reset(&mut self, permissions: HashSet<Permission>);
    fn permission_update(&mut self, args: Vec<PermissionUpdatedArg<Permission>>);
    fn permission_replace(
        &mut self,
        arg: PermissionReplacedArg<Permission>,
    ) -> PermissionReplacedArg<Permission>;
}

impl Permissions {
    // 通知
    // 如果需要通知宿主
    fn permission_register(&self) {
        if let Some(host) = self.host {
            let arg = PermissionReplacedArg {
                permissions: self.permissions.clone(),
                user_permissions: self.user_permissions.clone(),
                role_permissions: self.role_permissions.clone(),
                user_roles: self.user_roles.clone(),
            };
            let _ = async_execute(move || {
                ic_cdk::spawn(async move {
                    call_canister::<(PermissionReplacedArg<Permission>,), ()>(
                        host,
                        "business_permission_register",
                        (arg,),
                    )
                    .await
                    .unwrap();
                });
            });
        }
    }

    // 如果需要通知更新
    fn permission_notice_updated(&self, args: Vec<PermissionUpdatedArg<Permission>>) {
        let caller = caller();
        if let Some(host) = self.host {
            if caller != host {
                let args = args.clone();
                let _ = async_execute(move || {
                    ic_cdk::spawn(async move {
                        call_canister::<(Vec<PermissionUpdatedArg<Permission>>,), ()>(
                            host,
                            "business_permission_update",
                            (args,),
                        )
                        .await
                        .unwrap();
                    });
                });
            }
        }
        if let Some(listener) = self.listener {
            if caller != listener {
                let _ = async_execute(move || {
                    ic_cdk::spawn(async move {
                        call_canister::<(Vec<PermissionUpdatedArg<String>>,), ()>(
                            listener,
                            "permission_update",
                            (args.into_iter().map(|a| a.into()).collect(),),
                        )
                        .await
                        .unwrap();
                    });
                });
            }
        }
    }
    // 如果需要通知替换
    fn permission_notice_replaced(&self, arg: PermissionReplacedArg<Permission>) {
        let caller = caller();
        if let Some(host) = self.host {
            if caller != host {
                let arg = arg.clone();
                let _ = async_execute(move || {
                    ic_cdk::spawn(async move {
                        call_canister::<(PermissionReplacedArg<Permission>,), ()>(
                            host,
                            "business_permission_replace",
                            (arg,),
                        )
                        .await
                        .unwrap();
                    });
                });
            }
        }
        if let Some(listener) = self.listener {
            if caller != listener {
                let _ = async_execute(move || {
                    ic_cdk::spawn(async move {
                        call_canister::<(PermissionReplacedArg<String>,), ()>(
                            listener,
                            "permission_replace",
                            (arg.into(),),
                        )
                        .await
                        .unwrap();
                    });
                });
            }
        }
    }
}

impl Permissable for Permissions {
    // 查询
    fn permission_host_find(&self) -> Option<CanisterId> {
        self.host
    }
    fn permission_users(&self) -> HashSet<UserId> {
        let mut users: HashSet<&UserId> = self.user_roles.keys().collect();
        users.extend(self.user_permissions.keys());
        users.into_iter().map(|u| *u).collect()
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
    fn permission_host_update(&mut self, host: Option<CanisterId>, notice: bool) {
        self.host = host;
        if notice {
            self.permission_register();
        }
    }
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
    fn permission_update(&mut self, args: Vec<PermissionUpdatedArg<Permission>>) {
        let mut changed = false;

        for arg in args.iter() {
            match arg {
                PermissionUpdatedArg::UpdateUserPermission(user_id, permissions) => {
                    let exist = self.user_permissions.get(user_id);
                    if let Some(permissions) = permissions {
                        if let Some(exist) = exist {
                            if exist == permissions {
                                continue;
                            }
                        }
                    } else {
                        if let None = exist {
                            continue;
                        }
                    }
                    if let Some(permissions) = permissions {
                        self.user_permissions
                            .insert(user_id.clone(), permissions.clone());
                    } else {
                        self.user_permissions.remove(user_id);
                    }
                    changed = true;
                }
                PermissionUpdatedArg::UpdateRolePermission(role, permissions) => {
                    let exist = self.role_permissions.get(role);
                    if let Some(permissions) = permissions {
                        if let Some(exist) = exist {
                            if exist == permissions {
                                continue;
                            }
                        }
                    } else {
                        if let None = exist {
                            continue;
                        }
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
                    changed = true;
                }
                PermissionUpdatedArg::UpdateUserRole(user_id, roles) => {
                    // 先检查权限对不对
                    let roles = if let Some(roles) = roles {
                        Some(
                            roles
                                .iter()
                                .filter(|role| self.role_permissions.contains_key(*role))
                                .map(|r| r.clone())
                                .collect::<HashSet<String>>(),
                        )
                    } else {
                        None
                    };
                    let exist = self.user_roles.get(user_id);
                    if let Some(roles) = &roles {
                        if let Some(exist) = exist {
                            if exist == roles {
                                continue;
                            }
                        }
                    } else {
                        if let None = exist {
                            continue;
                        }
                    }
                    if let Some(roles) = roles {
                        self.user_roles.insert(*user_id, roles.clone());
                    } else {
                        self.user_roles.remove(user_id);
                    }
                    changed = true;
                }
            }
        }

        if changed {
            self.permission_notice_updated(args);
        }
    }
    fn permission_replace(
        &mut self,
        arg: PermissionReplacedArg<Permission>,
    ) -> PermissionReplacedArg<Permission> {
        let permissions = std::mem::replace(&mut self.permissions, arg.permissions.clone());
        let user_permissions =
            std::mem::replace(&mut self.user_permissions, arg.user_permissions.clone());
        let role_permissions =
            std::mem::replace(&mut self.role_permissions, arg.role_permissions.clone());
        let user_roles = std::mem::replace(&mut self.user_roles, arg.user_roles.clone());

        self.permission_notice_replaced(arg);

        PermissionReplacedArg {
            permissions,
            user_permissions,
            role_permissions,
            user_roles,
        }
    }
}

// 转换

impl From<PermissionUpdatedArg<Permission>> for PermissionUpdatedArg<String> {
    fn from(value: PermissionUpdatedArg<Permission>) -> Self {
        match value {
            PermissionUpdatedArg::UpdateUserPermission(user_id, permissions) => {
                PermissionUpdatedArg::UpdateUserPermission(
                    user_id,
                    permissions.and_then(|ps| {
                        Some(ps.into_iter().map(|p| p.name().to_string()).collect())
                    }),
                )
            }
            PermissionUpdatedArg::UpdateRolePermission(role, permissions) => {
                PermissionUpdatedArg::UpdateRolePermission(
                    role,
                    permissions.and_then(|ps| {
                        Some(ps.into_iter().map(|p| p.name().to_string()).collect())
                    }),
                )
            }
            PermissionUpdatedArg::UpdateUserRole(user_id, roles) => {
                PermissionUpdatedArg::UpdateUserRole(user_id, roles)
            }
        }
    }
}

impl From<PermissionReplacedArg<Permission>> for PermissionReplacedArg<String> {
    fn from(value: PermissionReplacedArg<Permission>) -> Self {
        PermissionReplacedArg {
            permissions: value
                .permissions
                .into_iter()
                .map(|p| p.name().to_string())
                .collect(),
            user_permissions: value
                .user_permissions
                .into_iter()
                .map(|(user_id, permissions)| {
                    (
                        user_id,
                        permissions
                            .into_iter()
                            .map(|p| p.name().to_string())
                            .collect(),
                    )
                })
                .collect(),
            role_permissions: value
                .role_permissions
                .into_iter()
                .map(|(role, permissions)| {
                    (
                        role,
                        permissions
                            .into_iter()
                            .map(|p| p.name().to_string())
                            .collect(),
                    )
                })
                .collect(),
            user_roles: value.user_roles,
        }
    }
}

impl PermissionUpdatedArg<String> {
    pub fn into_permission<F: Fn(&str) -> Permission>(
        self,
        f: F,
    ) -> PermissionUpdatedArg<Permission> {
        match self {
            PermissionUpdatedArg::UpdateUserPermission(user_id, permissions) => {
                PermissionUpdatedArg::UpdateUserPermission(
                    user_id,
                    permissions.and_then(|ps| Some(ps.into_iter().map(|p| f(&p)).collect())),
                )
            }
            PermissionUpdatedArg::UpdateRolePermission(role, permissions) => {
                PermissionUpdatedArg::UpdateRolePermission(
                    role,
                    permissions.and_then(|ps| Some(ps.into_iter().map(|p| f(&p)).collect())),
                )
            }
            PermissionUpdatedArg::UpdateUserRole(user_id, roles) => {
                PermissionUpdatedArg::UpdateUserRole(user_id, roles)
            }
        }
    }
}

impl PermissionReplacedArg<String> {
    pub fn into_permission<F: Fn(&str) -> Permission>(
        self,
        f: F,
    ) -> PermissionReplacedArg<Permission> {
        PermissionReplacedArg {
            permissions: self.permissions.iter().map(|p| f(&p)).collect(),
            user_permissions: self
                .user_permissions
                .into_iter()
                .map(|(user_id, permissions)| {
                    (user_id, permissions.iter().map(|p| f(&p)).collect())
                })
                .collect(),
            role_permissions: self
                .role_permissions
                .into_iter()
                .map(|(role, permissions)| (role, permissions.into_iter().map(|p| f(&p)).collect()))
                .collect(),
            user_roles: self.user_roles,
        }
    }
}

// 格式化

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Permitted(name) => f.write_str(&format!("Permitted({})", name)),
            Permission::Forbidden(name) => f.write_str(&format!("Forbidden({})", name)),
        }
    }
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Permissions {{ host: {}, listener: {}, permissions: {}, user_permissions: {}, role_permissions: {}, user_roles: {} }}",
            format_option(&self.host),
            format_option(&self.listener),
            format!(
                "[{}]",
                self.permissions
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "[{}]",
                self.user_permissions
                    .iter()
                    .map(|(user_id, permissions)| format!(
                        "{{user: {}, permissions: [{}]}}",
                        user_id.to_text(),
                        permissions
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "[{}]",
                self.role_permissions
                    .iter()
                    .map(|(role, permissions)| format!(
                        "{{role: {}, permissions: [{}]}}",
                        role,
                        permissions
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "[{}]",
                self.user_roles
                    .iter()
                    .map(|(user_id, roles)| format!(
                        "{{user: {}, roles: [{}]}}",
                        user_id.to_text(),
                        roles.iter().map(|s|s.clone()).collect::<Vec<_>>().join(",")
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        ))
    }
}

impl Display for PermissionUpdatedArg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UpdateUserPermission(user_id, permissions) => f.write_str(&format!(
                "update user: {} permissions: {}",
                user_id.to_text(),
                format_option_with_func(permissions, |permissions| format!(
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
                format_option_with_func(permissions, |permissions| format!(
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
                format_option_with_func(roles, |roles| format!(
                    "[{}]",
                    roles
                        .iter()
                        .map(|r| r.clone())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
            )),
        }
    }
}

impl Display for PermissionReplacedArg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "replace with permissions: [{}] user_permissions: [{}] role_permissions: [{}] user_roles: [{}]",
            self.permissions
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(","),
            self.user_permissions
                .iter()
                .map(|(user_id, permissions)| format!(
                    "({}, [{}])",
                    user_id.to_text(),
                    permissions
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(","),
            self.role_permissions
                .iter()
                .map(|(role, permissions)| format!(
                    "({}, [{}])",
                    role,
                    permissions
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(","),
            self.user_roles
                .iter()
                .map(|(user_id, roles)| format!(
                    "({}, [{}])",
                    user_id.to_text(),
                    roles
                        .iter()
                        .map(|p| p.clone())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(","),
        ))
    }
}
