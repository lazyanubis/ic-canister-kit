use crate::canister::{fetch_and_wrap_call_result, types::CanisterCallResult};

/// 生成随机数

// 得到随机数
// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-raw_rand
#[inline]
pub async fn random() -> CanisterCallResult<[u8; 32]> {
    let call_result = ic_cdk::api::management_canister::main::raw_rand().await;

    let random = fetch_and_wrap_call_result(
        crate::identity::CanisterId::anonymous(),
        "ic#raw_rand",
        call_result,
    )?;

    let mut data = [0; 32];

    data[..32].copy_from_slice(&random[..32]);

    Ok(data)
}

// ============= 节省生成随机数的时间 =============

// 如果每次只需要使用指定数量的随机数, 通过下面的方式节省随机数

pub struct RandomGenerator {
    random: [u8; 32],
    current: u8,
}

impl RandomGenerator {
    pub fn new() -> Self {
        RandomGenerator {
            random: [0; 32],
            current: 32,
        }
    }
    pub async fn next(&mut self, number: usize) -> CanisterCallResult<Vec<u8>> {
        let mut data = Vec::with_capacity(number);
        let mut remain = number;

        // 如果大于 32，就直接随机一个
        while remain > 32 {
            data.extend_from_slice(&random().await?);
            remain -= 32;
        }

        let available = 32 - self.current as usize;
        if remain <= available {
            let current = self.current as usize;
            data.extend_from_slice(&self.random[current..current + remain]);
            self.current += remain as u8;
        } else {
            // 剩下的全加入
            let current = self.current as usize;
            data.extend_from_slice(&self.random[current..current + available]);
            remain -= available;

            // 随机新的一组
            self.random = random().await?;
            self.current = 0;

            // 取出剩下的个数
            data.extend_from_slice(&self.random[0..remain]);
            self.current += remain as u8;
        }

        Ok(data)
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new()
    }
}
