use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::{
    canister::call::call_canister,
    identity::{caller, CanisterId, UserId},
    times::schedulable::async_execute,
};

use super::recordable::record_option;

/// 权限管理

// 被管理的用户类型
#[derive(candid::CandidType, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    // 授权类型 默认没有该权限 只有被加入的用户才有该权限
    Permitted(String),
    // 禁止类型 默认拥有该权限 如果被加入了就没有该权限了
    Forbidden(String),
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Permitted(name) => f.write_str(&format!("Permitted({})", name)),
            Permission::Forbidden(name) => f.write_str(&format!("Forbidden({})", name)),
        }
    }
}

impl Hash for Permission {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Permission::Permitted(name) => name.hash(state),
            Permission::Forbidden(name) => name.hash(state),
        }
    }
}

impl Permission {
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
    fn permit(&self, users: &mut HashSet<UserId>, user_id: &UserId) -> bool {
        match self {
            Permission::Permitted(_) => users.insert(user_id.clone()),
            Permission::Forbidden(_) => users.remove(user_id),
        }
    }
    fn forbid(&self, users: &mut HashSet<UserId>, user_id: &UserId) -> bool {
        match self {
            Permission::Permitted(_) => users.remove(user_id),
            Permission::Forbidden(_) => users.insert(user_id.clone()),
        }
    }
    fn has_permission(&self, users: &HashSet<UserId>, user_id: &UserId) -> bool {
        match &self {
            Permission::Permitted(_) => users.contains(&user_id),
            Permission::Forbidden(_) => !users.contains(&user_id),
        }
    }
}

// 单个权限对象
pub type PermissionEntry<'p> = (Permission, Cow<'p, HashSet<UserId>>);

// 多个权限对象
#[derive(candid::CandidType, serde::Deserialize, Debug, Default)]
pub struct Permissions {
    pub host: Option<CanisterId>, // 本数据有宿主, 一旦变更调用宿主的业务接口
    pub listener: Option<CanisterId>, // 本数据有监听者, 一旦变更调用监听者的通用接口
    pub data: HashMap<Permission, HashSet<UserId>>,
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Permissions {{ host: {}, listener: {}, data: {} }}",
            record_option(&self.host),
            record_option(&self.listener),
            format!(
                "{:?}",
                self.data
                    .iter()
                    .map(|(p, u)| (p, u.iter().map(|id| id.to_text()).collect::<Vec<_>>()))
                    .collect::<HashMap<_, _>>()
            ),
        ))
    }
}

pub trait Permissable {
    // 查询
    fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool;
    fn permission_owned(&self, user_id: &UserId) -> HashMap<Permission, bool>;
    fn permission_users(&self) -> HashSet<UserId>;
    fn permission_entry<'p>(&'p self, permission: Permission) -> PermissionEntry<'p>; // 获取某权限的所有用户
    fn permission_host_find(&self) -> Option<CanisterId>;
    // 通知
    fn permission_register(&self);
    fn permission_notice_updated(&self, args: Vec<PermissionUpdatedArg<Permission>>);
    fn permission_notice_replaced(&self, args: Vec<PermissionReplacedArg<Permission>>);
    // 修改
    fn permission_replace_assure(&mut self, permissions: HashSet<Permission>);
    fn permission_update(&mut self, args: Vec<PermissionUpdatedArg<Permission>>);
    fn permission_replace(
        &mut self,
        args: Vec<PermissionReplacedArg<Permission>>,
    ) -> Vec<PermissionReplacedArg<Permission>>;
    fn permission_host_replace(&mut self, host: Option<CanisterId>);
}

// 初始化某权限数据
fn assure_permission<'a, 'b>(
    permissions: &'a mut Permissions,
    permission: &Permission,
) -> &'a mut HashSet<UserId> {
    if !permissions.data.contains_key(&permission) {
        // 不存在该权限则初始化
        permissions.data.insert(permission.clone(), HashSet::new());
    }
    permissions.data.get_mut(&permission).unwrap()
}

impl Permissable for Permissions {
    // 查询
    // 判断是否有某权限
    fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool {
        if let Some(users) = self.data.get(permission) {
            return permission.has_permission(users, user_id); // 存在则比较
        }
        // 不存在则分情况判断
        match permission {
            Permission::Permitted(_) => false,
            Permission::Forbidden(_) => true,
        }
    }

    // 查询单个人的所有权限情况
    fn permission_owned(&self, user_id: &UserId) -> HashMap<Permission, bool> {
        self.data
            .keys()
            .map(|permission| (permission.clone(), self.permission_has(user_id, permission)))
            .collect()
    }

    // 查询所有涉及的用户
    fn permission_users(&self) -> HashSet<UserId> {
        self.data
            .values()
            .flat_map(|users| users)
            .map(|u| u.clone())
            .collect()
    }

    // 获取某权限的所有用户
    fn permission_entry<'p>(&'p self, permission: Permission) -> PermissionEntry<'p> {
        let exist = self.data.get(&permission);
        if let Some(users) = exist {
            return (permission, Cow::Borrowed(users)); // 存在则返回
        }
        // 不存在则分情况判断
        (permission, Cow::Owned(HashSet::with_capacity(0)))
    }

    fn permission_host_find(&self) -> Option<CanisterId> {
        self.host
    }

    // 通知
    // 如果需要通知宿主
    fn permission_register(&self) {
        if let Some(host) = self.host {
            let args: Vec<PermissionReplacedArg<Permission>> = self
                .data
                .keys()
                .into_iter()
                .map(|permission| self.permission_entry(permission.clone()))
                .map(|(permission, users)| {
                    PermissionReplacedArg(
                        permission.clone(),
                        users.iter().map(|p| p.clone()).collect(),
                    )
                })
                .collect();
            let _ = async_execute(move || {
                ic_cdk::spawn(async move {
                    call_canister::<(Vec<PermissionReplacedArg<Permission>>,), ()>(
                        host,
                        "business_permission_register",
                        (args,),
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
    fn permission_notice_replaced(&self, args: Vec<PermissionReplacedArg<Permission>>) {
        let caller = caller();
        if let Some(host) = self.host {
            if caller != host {
                let args = args.clone();
                let _ = async_execute(move || {
                    ic_cdk::spawn(async move {
                        call_canister::<(Vec<PermissionReplacedArg<Permission>>,), ()>(
                            host,
                            "business_permission_replace",
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
                        call_canister::<(Vec<PermissionReplacedArg<String>>,), ()>(
                            listener,
                            "permission_replace",
                            (args
                                .into_iter()
                                .map(|a| a.into())
                                .collect::<Vec<PermissionReplacedArg<String>>>(),),
                        )
                        .await
                        .unwrap();
                    });
                });
            }
        }
    }

    // 修改
    fn permission_replace_assure(&mut self, permissions: HashSet<Permission>) {
        let mut removed = Vec::new();
        for permission in self.data.keys() {
            if !permissions.contains(permission) {
                removed.push(permission.clone());
            }
        }
        for permission in removed {
            self.data.remove(&permission);
        }
    }
    // 更新权限
    fn permission_update(&mut self, args: Vec<PermissionUpdatedArg<Permission>>) {
        let mut changed = false;

        for PermissionUpdatedArg(user_id, permission, grant) in args.iter() {
            let users: &mut HashSet<candid::Principal> = assure_permission(self, &permission); // 确保有这个权限
            let result = if *grant {
                permission.permit(users, &user_id)
            } else {
                permission.forbid(users, &user_id)
            };
            if result {
                changed = true
            }
        }

        if changed {
            self.permission_notice_updated(args);
        }
    }
    // 替换权限
    fn permission_replace(
        &mut self,
        args: Vec<PermissionReplacedArg<Permission>>,
    ) -> Vec<PermissionReplacedArg<Permission>> {
        let old = std::mem::replace(
            &mut self.data,
            args.iter().map(|s| (s.0.clone(), s.1.clone())).collect(),
        );
        self.permission_notice_replaced(args);
        old.into_iter()
            .map(|(permission, users)| PermissionReplacedArg(permission, users))
            .collect()
    }

    fn permission_host_replace(&mut self, host: Option<CanisterId>) {
        self.host = host;
        self.permission_register();
    }
}

#[derive(candid::CandidType, serde::Deserialize, Debug)]
pub struct PermissionUsers {
    pub permission: Permission,
    pub users: HashSet<UserId>,
}

impl PermissionUsers {
    pub fn new(permission: Permission, users: HashSet<UserId>) -> Self {
        PermissionUsers { permission, users }
    }
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct PermissionUpdatedArg<T>(pub UserId, pub T, pub bool);

impl From<PermissionUpdatedArg<Permission>> for PermissionUpdatedArg<String> {
    fn from(value: PermissionUpdatedArg<Permission>) -> Self {
        PermissionUpdatedArg(value.0, value.1.name().to_string(), value.2)
    }
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct PermissionReplacedArg<T>(pub T, pub HashSet<UserId>);

impl<T> PermissionReplacedArg<T> {
    pub fn new(permission: T, users: HashSet<UserId>) -> Self {
        PermissionReplacedArg(permission, users)
    }
}

impl From<PermissionReplacedArg<Permission>> for PermissionReplacedArg<String> {
    fn from(value: PermissionReplacedArg<Permission>) -> Self {
        PermissionReplacedArg(value.0.name().to_string(), value.1)
    }
}

impl Display for PermissionUpdatedArg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\"{} {} {}\"",
            self.0.to_text(),
            if self.2 { "PERMIT" } else { "FORBID" },
            self.1
        ))
    }
}

impl Display for PermissionReplacedArg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} {}",
            self.0,
            format!(
                "[{}]",
                self.1
                    .iter()
                    .map(|user| format!("\"{}\"", user.to_text()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        ))
    }
}

impl Display for PermissionUpdatedArg<Permission> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\"{} {} {}\"",
            self.0.to_text(),
            if self.2 { "PERMIT" } else { "FORBID" },
            self.1
        ))
    }
}

impl Display for PermissionReplacedArg<Permission> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} {}",
            self.0,
            format!(
                "[{}]",
                self.1
                    .iter()
                    .map(|user| format!("\"{}\"", user.to_text()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        ))
    }
}
