use sha1::{Digest, Sha1};

pub fn get_sha1_hash(u8_vector: &Vec<u8>) -> [u8; 20] {
    Sha1::digest(u8_vector).into()
}
