// 上传数据缓存

const MAX: u32 = 1024 * 1024 * 2;

#[derive(candid::CandidType, serde::Deserialize, Debug)]
pub struct UploadCache(Vec<u8>);

impl UploadCache {
    pub fn extend(&mut self, data: &[u8]) {
        self.0.extend_from_slice(data);
    }
    pub fn clear(&mut self) {
        std::mem::take(&mut self.0);
    }
    pub fn fetch(&mut self) -> Vec<u8> {
        let cache = std::mem::take(&mut self.0);
        cache
    }
    pub fn fetch_slice(&self, start: usize, end: usize) -> Vec<u8> {
        if end < start {
            panic!("Start can not be less than end");
        }
        if end < start + MAX as usize {
            panic!("The max range is {}", MAX);
        }
        if self.0.len() <= end {
            panic!("The length of cache is less than end");
        }

        self.0[start..end].to_vec()
    }
}
