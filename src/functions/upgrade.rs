//! 升级数据版本

// ================== 功能 ==================

/// 版本升级
pub trait Upgrade<T> {
    /// 进行升级
    fn upgrade(&mut self, arg: T);
}

/// 状态升级
pub trait StateUpgrade<T>: Upgrade<T> {
    /// 当前版本
    fn version(&self) -> u32;

    /// 获取指定版本的默认数据
    fn from_version(version: u32) -> Self;
}
