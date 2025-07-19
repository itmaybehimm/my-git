use hex::encode;
use sha1::{Digest, Sha1};
pub trait GitObject {
    fn object_type(&self) -> &'static str;
    fn content_bytes(&self) -> Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let content = self.content_bytes();
        let header = format!("{} {}\0", self.object_type(), content.len());
        let mut data = header.into_bytes();
        data.extend_from_slice(&content);
        data
    }
    fn hash_bytes(&self) -> [u8; 20] {
        Sha1::digest(&self.serialize()).into()
    }

    fn hash_str(&self) -> String {
        encode(self.hash_bytes())
    }
}
