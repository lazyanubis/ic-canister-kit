//! 生成随机数

use crate::canister::types::CanisterCallResult;

/// 得到随机数
/// <https://docs.internetcomputer.org/references/management-canister/#raw_rand>
#[inline]
pub async fn random() -> CanisterCallResult<[u8; 32]> {
    let call_result = ic_cdk_management_canister::raw_rand().await;

    let random = call_result.map_err(|err| crate::canister::types::CanisterCallError {
        canister_id: crate::identity::CanisterId::management_canister(),
        method: "ic#raw_rand".to_string(),
        message: err.to_string(),
    })?;

    let mut data = [0; 32];

    data[..32].copy_from_slice(&random[..32]);

    Ok(data)
}

// ============= 节省生成随机数的时间 =============

// 如果每次只需要使用指定数量的随机数, 通过下面的方式节省随机数

/// 随机数生产对象
#[derive(Debug)]
pub struct RandomGenerator {
    random: [u8; 32],
    cursor: u8,
}

impl RandomGenerator {
    /// 构建对象
    pub fn new() -> Self {
        RandomGenerator {
            random: [0; 32],
            cursor: 32,
        }
    }
    /// 下一组随机数
    pub async fn next(&mut self, number: usize) -> CanisterCallResult<Vec<u8>> {
        let mut data = Vec::with_capacity(number);
        let mut remain = number;

        // 如果大于 32，就直接随机一个
        while remain > 32 {
            data.extend_from_slice(&random().await?);
            remain -= 32;
        }

        let available = 32 - self.cursor as usize;
        if remain <= available {
            let cursor = self.cursor as usize;
            data.extend_from_slice(&self.random[cursor..cursor + remain]);
            self.cursor += remain as u8;
        } else {
            // 剩下的全加入
            let cursor = self.cursor as usize;
            data.extend_from_slice(&self.random[cursor..cursor + available]);
            remain -= available;

            // 随机新的一组
            self.random = random().await?;
            self.cursor = 0;

            // 取出剩下的个数
            data.extend_from_slice(&self.random[0..remain]);
            self.cursor += remain as u8;
        }

        Ok(data)
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new()
    }
}
