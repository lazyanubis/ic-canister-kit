use super::super::business::*;
use super::types::*;

#[allow(unused_variables)]
impl Business for InnerState {
    fn business_files(&self) -> Vec<QueryFile> {
        self.files()
    }
    fn business_download(&self, path: String) -> Vec<u8> {
        self.download(path)
    }
    fn business_download_by(&self, path: String, offset: u64, offset_end: u64) -> Vec<u8> {
        self.download_by(path, offset, offset_end)
    }

    fn business_upload(&mut self, args: Vec<UploadingArg>) {
        for arg in args {
            self.put_uploading(arg)
        }
    }

    fn business_delete(&mut self, names: Vec<String>) {
        for name in names {
            self.clean_uploading(&name);
            self.clean_file(&name);
        }
    }

    fn business_assets_files(&self) -> &HashMap<String, AssetFile> {
        &self.heap.business.files
    }
    fn business_assets_assets(&self) -> &StableBTreeMap<HashDigest, AssetData> {
        &self.stable_business.assets
    }

    fn business_assets_insert(&mut self, hash: HashDigest, asset: AssetData) {
        if !self.stable_business.assets.contains_key(&hash) {
            self.stable_business.assets.insert(hash, asset);
        }
    }
    fn business_assets_remove(&mut self, hash: &HashDigest) {
        self.stable_business.assets.remove(hash);
    }
    fn business_assets_get(&self, hash: &HashDigest) -> Option<AssetData> {
        self.stable_business.assets.get(hash)
    }
}
