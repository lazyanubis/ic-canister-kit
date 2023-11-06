use crate::{identity::CanisterId, times::TimestampNanos};

#[cfg(feature = "stable_initial")]
pub use super::initial::Initial;

#[cfg(feature = "stable_upgrade")]
pub use super::upgrade::Upgrade;

#[cfg(feature = "stable_maintainable")]
pub use super::maintainable::{Maintainable, Maintaining, MaintainingReason};

#[cfg(feature = "stable_permissable")]
pub use super::permissable::{
    Permissable, Permission, PermissionReplacedArg, PermissionUpdatedArg, Permissions,
};

#[cfg(feature = "stable_recordable")]
pub use super::recordable::{
    MigratedRecords, Record, RecordLevel, RecordSearch, Recordable, Records,
};

#[cfg(feature = "stable_uploads")]
pub use super::uploads::UploadCache;

#[cfg(feature = "stable_hashmap")]
pub use super::hashmap::{CustomHashMap, CustomHashMapState};

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct CanisterInitialArg {
    pub permission_host: Option<CanisterId>,
    pub record_collector: Option<CanisterId>,
    pub schedule: Option<TimestampNanos>,
}

impl CanisterInitialArg {
    pub fn none() -> Self {
        CanisterInitialArg {
            permission_host: None,
            record_collector: None,
            schedule: None,
        }
    }
}
