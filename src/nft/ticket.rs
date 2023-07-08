use candid::CandidType;
use serde::Deserialize;

use crate::stable::Stable;
use crate::times::{now, Duration, Timestamp};

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct ForbiddenDuration {
    start: Timestamp,
    end: Timestamp,
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct NftTicket {
    activity_start: Timestamp, // 开始前不给看到秘钥, 开始后所有者可以看到秘钥
    activity_end: Timestamp,   // 结束后所有人都可以看到秘钥
    transfer_forbidden: Vec<ForbiddenDuration>, // 该时间段内,不允许交易
}

pub type NftTicketState = (Timestamp, Timestamp, Vec<ForbiddenDuration>);

impl Stable<NftTicketState, NftTicketState> for NftTicket {
    fn save(&mut self) -> NftTicketState {
        let activity_start = std::mem::take(&mut self.activity_start);
        let activity_end = std::mem::take(&mut self.activity_end);
        let transfer_forbidden = std::mem::take(&mut self.transfer_forbidden);
        (activity_start, activity_end, transfer_forbidden)
    }

    fn restore(&mut self, state: NftTicketState) {
        let _ = std::mem::replace(&mut self.activity_start, state.0);
        let _ = std::mem::replace(&mut self.activity_end, state.1);
        let _ = std::mem::replace(&mut self.transfer_forbidden, state.2);
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub enum NftTicketStatus {
    NoBody(Duration),            // 当前所有人都看不到, 里面是距离开始时间
    InvalidToken,                // 无效的 id
    Forbidden,                   // 无权查看
    Owner(Duration, String),     // 当前所有者能看到, 里面是距离结束时间
    Anonymous(Duration, String), // 会议结束后所有人都可以看, 里面是结束多长时间了
}

impl NftTicket {
    pub fn can_transfer(&self) -> bool {
        let now = now();
        for ForbiddenDuration { start, end } in self.transfer_forbidden.iter() {
            if start <= &now && &now < end {
                return false;
            }
        }
        true
    }
    pub fn ticket_status(&self) -> NftTicketStatus {
        let now = now();
        if now < self.activity_start {
            return NftTicketStatus::NoBody(self.activity_start - now); // 还没到开放的时间
        } else if now < self.activity_end {
            // ! 需要检查权限
            return NftTicketStatus::Owner(self.activity_end - now, format!(""));
        } else {
            return NftTicketStatus::Anonymous(now - self.activity_end, format!(""));
            // 无需检查权限
        }
    }
}
