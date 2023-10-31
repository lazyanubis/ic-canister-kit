/// 升级数据版本

pub trait Upgrade {
    fn version(&self) -> u32;

    fn upgrade(&mut self);
}
