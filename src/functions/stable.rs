//! 升级代码需要持久化

// ================== 功能 ==================

/// 版本升级
pub trait StableHeap {
    /// 升级前，将内存中需要持久化的数据转换为字节数组
    fn heap_to_bytes(&self) -> Vec<u8>;

    /// 升级后，将字节数组还原为内存中需要持久化的数据
    fn heap_from_bytes(&mut self, bytes: &[u8]);
}

// ================== 工具方法 ==================

/// 序列化
pub fn to_bytes<T: ?Sized + serde::Serialize>(value: &T) -> Vec<u8> {
    let mut bytes = vec![];
    #[allow(clippy::unwrap_used)] // ? SAFETY
    ciborium::ser::into_writer(value, &mut bytes).unwrap();
    bytes
}

/// 序列化
pub fn from_bytes<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T {
    #[allow(clippy::expect_used)] // ? SAFETY
    ciborium::de::from_reader(bytes).expect("deserialization must succeed.")
}
