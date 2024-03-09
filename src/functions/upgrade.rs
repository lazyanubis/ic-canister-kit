/// 升级数据版本

// ================== 功能 ==================

/// 版本升级
pub trait Upgrade {
    /// 当前版本
    fn version(&self) -> u32;

    /// 进行升级
    fn upgrade(&mut self);
}
