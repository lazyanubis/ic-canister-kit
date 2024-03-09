/// 初始化数据

// ================== 功能 ==================

/// 初始化
pub trait Initial<T> {
    /// 初始化
    fn init(&mut self, arg: T);
}
