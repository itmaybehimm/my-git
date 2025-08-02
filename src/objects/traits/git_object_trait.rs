use crate::utils::get_sha1_hash;
use hex::encode;

pub trait GitObject {
    fn get_object_type(&self) -> &'static str;
    fn get_content_bytes(&self) -> Vec<u8>;
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
}
