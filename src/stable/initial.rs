/// 初始化数据

pub trait Initial<T> {
    fn init(&mut self, arg: T);
}
