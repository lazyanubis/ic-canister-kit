use crate::times::{now, Duration, Timestamp};
use crate::types::NFTOwnable;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ForbiddenDuration {
    start: Timestamp,
    end: Timestamp,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct NftTicket {
    activity_start: Timestamp, // 开始前不给看到秘钥, 开始后所有者可以看到秘钥
    activity_end: Timestamp,   // 结束后所有人都可以看到秘钥
    transfer_forbidden: Vec<ForbiddenDuration>, // 该时间段内,不允许交易
}

//  所有人不可见  ownable  所有者可见  opened  匿名可见
// ----------------> ----------------> -------------->
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum NftTicketStatus {
    NoBody(Duration),                // 所有人不可见, 数字是 ownable - now
    InvalidToken,                    // 无效的 id
    Forbidden(Duration),             // 无权查看 数字是 opened - now
    Owner(Duration, NFTOwnable),     // 所有者能可见 数字是 opened - now
    Anonymous(Duration, NFTOwnable), // 匿名可见 数字是 now - opened
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
            // ! 需要检查权限, 权限通过后, 放入数据
            return NftTicketStatus::Owner(self.activity_end - now, NFTOwnable::None);
        } else {
            // 无需检查权限
            return NftTicketStatus::Anonymous(now - self.activity_end, NFTOwnable::None);
        }
    }
    pub fn set_activity_start(&mut self, start: Timestamp) {
        self.activity_start = start;
    }
    pub fn set_activity_end(&mut self, end: Timestamp) {
        self.activity_end = end;
    }
    pub fn set_transfer_forbidden(&mut self, forbidden: Vec<ForbiddenDuration>) {
        self.transfer_forbidden = forbidden;
    }

    pub fn get_activity(&self) -> (Timestamp, Timestamp) {
        (self.activity_start, self.activity_end)
    }
    pub fn get_transfer_forbidden(&self) -> Vec<ForbiddenDuration> {
        self.transfer_forbidden.clone()
    }
}
