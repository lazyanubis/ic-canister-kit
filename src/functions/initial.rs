/// 初始化数据

// ================== 功能 ==================

pub trait Initial<T> {
    fn init(&mut self, arg: T);
}
