use super::stable::Stable;

#[derive(Debug, Default)]
pub struct Maintainable {
    maintaining: bool,
}

pub type MaintainableState = (bool,);

impl Stable<MaintainableState, MaintainableState> for Maintainable {
    fn save(&mut self) -> MaintainableState {
        let maintaining = std::mem::take(&mut self.maintaining);
        (maintaining,)
    }

    fn restore(&mut self, state: MaintainableState) {
        let _ = std::mem::replace(&mut self.maintaining, state.0);
    }
}

impl Maintainable {
    pub fn must_be_running(&self) {
        if self.maintaining {
            panic!("System is maintaining");
        }
    }
    pub fn is_maintaining(&self) -> bool {
        self.maintaining
    }
    pub fn set_maintaining(&mut self, maintaining: bool) {
        self.maintaining = maintaining;
    }
}
