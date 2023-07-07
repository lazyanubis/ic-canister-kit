pub async fn random() -> [u8; 32] {
    let random = ic_cdk::api::management_canister::main::raw_rand()
        .await
        .unwrap()
        .0;

    let mut data = [0; 32];
    for i in 0..32 {
        data[i] = random[i];
    }
    data
}

// ============= 节省生成随机数的时间 =============

pub struct RandomProduce {
    random: [u8; 32],
    current: u8,
}

impl RandomProduce {
    pub fn new() -> Self {
        RandomProduce {
            random: [0; 32],
            current: 32,
        }
    }
    pub async fn next(&mut self, number: u8) -> Vec<u8> {
        let mut data = Vec::new();
        let mut remain = number;

        // 如果大于 32，就直接随机一个
        while remain > 32 {
            data.extend_from_slice(&random().await);
            remain -= 32;
        }

        let available = 32 - self.current;
        if remain <= available {
            for i in 0..remain {
                data.push(self.random[(self.current + i) as usize]);
            }
            self.current += remain;
        } else {
            for i in 0..available {
                data.push(self.random[(self.current + i) as usize]);
            }
            remain -= available;

            self.random = random().await;
            self.current = 0;

            for i in 0..remain {
                data.push(self.random[(self.current + i) as usize]);
            }
            self.current += remain;
        }

        data
    }
}
