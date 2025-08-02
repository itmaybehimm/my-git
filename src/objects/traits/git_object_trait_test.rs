#[cfg(test)]
mod tests {
    use super::super::git_object_trait::*;

    struct MockGitObject;

    impl GitObject for MockGitObject {
        fn get_object_type(&self) -> &'static str {
            "blob"
        }

        fn get_content_bytes(&self) -> Vec<u8> {
            b"hello git".to_vec()
        }
    }

    #[test]
    fn test_serialize() {
        let obj = MockGitObject;

        assert_eq!(obj.serialize(), b"blob 9\0hello git");

        assert_eq!(
            obj.serialize(),
            [98, 108, 111, 98, 32, 57, 0, 104, 101, 108, 108, 111, 32, 103, 105, 116,].to_vec()
        );
    }

    #[test]
    fn test_get_hash_bytes() {
        let obj = MockGitObject;
        let expected_bytes: [u8; 20] = hex::decode("f09e9c379f5fe8f4ce718641c356df87906d87a6")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(obj.get_hash_bytes(), expected_bytes);
    }

    #[test]
    fn test_get_hash_str() {
        let obj = MockGitObject;
        assert_eq!(
            obj.get_hash_str(),
            "f09e9c379f5fe8f4ce718641c356df87906d87a6"
        );
    }
}
