use super::stable::Stable;

const MAX: u32 = 1024 * 1024 * 2;

#[derive(Debug, Default)]
pub struct UploadCache {
    cache: Vec<u8>,
}

pub type UploadCacheState = (Vec<u8>,);

impl Stable<UploadCacheState, UploadCacheState> for UploadCache {
    fn save(&mut self) -> UploadCacheState {
        let cache = std::mem::take(&mut self.cache);
        (cache,)
    }

    fn restore(&mut self, state: UploadCacheState) {
        let _ = std::mem::replace(&mut self.cache, state.0);
    }
}

impl UploadCache {
    pub fn extend(&mut self, data: &[u8]) {
        self.cache.extend_from_slice(data);
    }
    pub fn clear(&mut self) {
        std::mem::take(&mut self.cache);
    }
    pub fn fetch(&mut self) -> Vec<u8> {
        let cache = std::mem::take(&mut self.cache);
        cache
    }
    pub fn fetch_slice(&self, start: usize, end: usize) -> Vec<u8> {
        if end < start {
            panic!("Start can not be less than end");
        }
        if end < start + MAX as usize {
            panic!("The max range is {}", MAX);
        }
        if self.cache.len() <= end {
            panic!("The length of cache is less than end");
        }

        self.cache[start..end].to_vec()
    }
}
