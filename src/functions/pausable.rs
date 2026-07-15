//! 维护状态

// ================== 功能 ==================

/// 维护原因
pub trait Reasonable {
    /// 维护原因
    fn message(&self) -> &str;
}

/// 维护记录
pub trait Pausable<Reason: Reasonable> {
    // 查询

    /// 查询维护状态
    fn pause_query(&self) -> &Option<Reason>;

    // 修改

    /// 修改维护状态
    fn pause_replace(&mut self, reason: Option<Reason>);

    // 默认方法

    /// 是否维护中
    fn pause_is_paused(&self) -> bool {
        self.pause_query().is_some()
    }
    /// 是否正常运行
    fn pause_is_running(&self) -> bool {
        !self.pause_is_paused()
    }
    /// 正常运行中才能继续
    fn pause_must_be_running(&self) -> Result<(), String> {
        if let Some(reason) = &self.pause_query() {
            return Err(format!("Canister is paused: {}", reason.message()));
        }
        Ok(())
    }
    /// 维护中才能继续
    fn pause_must_be_paused(&self) -> Result<(), String> {
        if self.pause_is_running() {
            return Err("Canister is running. Not paused.".into());
        }
        Ok(())
    }
}

// ================== 简单实现 ==================

/// 维护功能简单实现
pub mod basic {
    use std::fmt::Display;

    use candid::CandidType;
    use serde::{Deserialize, Serialize};

    use crate::{
        functions::types::{Pausable, Reasonable},
        types::TimestampNanos,
    };

    /// 维护原因对象
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct PauseReason {
        /// 进入维护状态的时间
        #[serde(alias = "timestamp_nanos")]
        pub paused_at: TimestampNanos,

        /// 维护原因
        pub message: String,
    }

    impl Display for PauseReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{:?}", self))
        }
    }

    impl std::error::Error for PauseReason {}

    impl Reasonable for PauseReason {
        fn message(&self) -> &str {
            &self.message
        }
    }

    impl PauseReason {
        /// 构造维护原因
        pub fn new(message: String) -> Self {
            PauseReason {
                paused_at: crate::times::now(),
                message,
            }
        }
    }

    /// 记录维护状态
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
    pub struct Pause(Option<PauseReason>);

    impl Pausable<PauseReason> for Pause {
        // 查询
        fn pause_query(&self) -> &Option<PauseReason> {
            &self.0
        }
        // 修改
        // 设置维护状态
        fn pause_replace(&mut self, reason: Option<PauseReason>) {
            self.0 = reason;
        }
    }

    #[cfg(test)]
    mod tests {
        use ciborium::value::Value;
        use serde::Serialize;

        use super::PauseReason;
        use crate::types::TimestampNanos;

        #[derive(Serialize)]
        struct LegacyPauseReason {
            timestamp_nanos: TimestampNanos,
            message: String,
        }

        #[test]
        fn deserializes_legacy_pause_time_and_serializes_current_name() {
            let legacy = LegacyPauseReason {
                timestamp_nanos: TimestampNanos::from(42),
                message: "maintenance".to_string(),
            };
            let mut legacy_cbor = Vec::new();
            ciborium::ser::into_writer(&legacy, &mut legacy_cbor).unwrap();
            let decoded: PauseReason = ciborium::de::from_reader(legacy_cbor.as_slice()).unwrap();
            assert_eq!(decoded.paused_at, TimestampNanos::from(42));

            let mut current_cbor = Vec::new();
            ciborium::ser::into_writer(&decoded, &mut current_cbor).unwrap();
            let current: Value = ciborium::de::from_reader(current_cbor.as_slice()).unwrap();
            let Value::Map(entries) = current else {
                panic!("expected a CBOR map")
            };
            let keys: Vec<&str> = entries
                .iter()
                .filter_map(|(key, _)| match key {
                    Value::Text(key) => Some(key.as_str()),
                    _ => None,
                })
                .collect();
            assert!(keys.contains(&"paused_at"));
            assert!(!keys.contains(&"timestamp_nanos"));
        }
    }
}
