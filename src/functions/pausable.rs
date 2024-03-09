/// 维护状态

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
