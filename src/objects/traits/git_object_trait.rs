use crate::utils::{get_sha1_hash, Compressor, FileUtils};
use hex::encode;
use std::io;
use std::path::Path;

pub trait GitObject {
    fn get_object_type(&self) -> &'static str;
    fn get_content_bytes(&self) -> Vec<u8>;
    fn get_compressor(&self) -> &dyn Compressor;
    fn get_file_utils(&self) -> &dyn FileUtils;

    fn serialize(&self) -> Vec<u8> {
        let content = self.get_content_bytes();
        let header = format!("{} {}\0", self.get_object_type(), content.len());
        let mut data = header.into_bytes();
        data.extend_from_slice(&content);
        data
    }

    fn get_hash_bytes(&self) -> [u8; 20] {
        get_sha1_hash(&self.serialize())
    }

    fn get_hash_str(&self) -> String {
        encode(self.get_hash_bytes())
    }

    fn compress_and_write_to_disk(&self, base_path: &Path) -> io::Result<()> {
        let hash = self.get_hash_str();
        let (dir, file) = hash.split_at(2);

        let dir_path = base_path.join(dir);
        std::fs::create_dir_all(&dir_path)?;

        let file_path = dir_path.join(file);

        let compressed = self.get_compressor().compress(&self.serialize())?;

        self.get_file_utils().write_all(&file_path, &compressed)?;

        Ok(())
    }
}
