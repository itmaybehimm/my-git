#[cfg(test)]
mod tests {
    use crate::utils::get_sha1_hash;

    #[test]
    fn test_get_sha1_hash() {
        let string = "test";
        let expected_bytes: [u8; 20] = hex::decode("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(get_sha1_hash(&string.as_bytes().to_vec()), expected_bytes);
    }
}
