use crate::utils::get_sha1_hash;

pub fn mock_hash(string: Option<String>) -> [u8; 20] {
    let val = string.unwrap_or("test".to_string());

    get_sha1_hash(&val.as_bytes().to_vec())
}
