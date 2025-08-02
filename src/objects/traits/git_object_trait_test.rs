#[cfg(test)]
mod tests {
    use super::super::git_object_trait::*;
    use crate::utils::{Compressor, FileUtils, MockCompressor, MockFileUtils};
    use std::path::PathBuf;
    use tempfile::tempdir;

    pub struct MockGitObject {
        pub compressor: Box<dyn Compressor>,
        pub file_utils: Box<dyn FileUtils>,
    }

    impl GitObject for MockGitObject {
        fn get_object_type(&self) -> &'static str {
            "blob"
        }

        fn get_content_bytes(&self) -> Vec<u8> {
            b"hello git".to_vec()
        }

        fn get_compressor(&self) -> &dyn Compressor {
            self.compressor.as_ref()
        }

        fn get_file_utils(&self) -> &dyn FileUtils {
            self.file_utils.as_ref()
        }
    }

    #[test]
    fn test_serialize() {
        let obj = MockGitObject {
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };

        assert_eq!(obj.serialize(), b"blob 9\0hello git");

        assert_eq!(
            obj.serialize(),
            [98, 108, 111, 98, 32, 57, 0, 104, 101, 108, 108, 111, 32, 103, 105, 116,].to_vec()
        );
    }

    #[test]
    fn test_get_hash_bytes() {
        let obj = MockGitObject {
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };

        let expected_bytes: [u8; 20] = hex::decode("f09e9c379f5fe8f4ce718641c356df87906d87a6")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(obj.get_hash_bytes(), expected_bytes);
    }

    #[test]
    fn test_get_hash_str() {
        let obj = MockGitObject {
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };

        assert_eq!(
            obj.get_hash_str(),
            "f09e9c379f5fe8f4ce718641c356df87906d87a6"
        );
    }

    #[test]
    fn test_compress_and_write_to_disk() {
        let expected_serialized_data = b"blob 9\0hello git".to_vec();
        let expected_compressed_data_compressor = b"compressed".to_vec();
        let expected_compressed_data_file = expected_compressed_data_compressor.clone();
        let expected_hash = "f09e9c379f5fe8f4ce718641c356df87906d87a6";
        let (dir, file) = expected_hash.split_at(2);

        let base_path = tempdir().unwrap();
        let expected_file_path: PathBuf = base_path.path().join(dir).join(file);

        let mut mock_compressor = MockCompressor::new();

        let mut mock_file_utils = MockFileUtils::new();

        mock_compressor
            .expect_compress()
            .withf(move |input| input == &expected_serialized_data)
            .returning(move |_| Ok(expected_compressed_data_compressor.clone()))
            .times(1);

        mock_file_utils
            .expect_write_all()
            .withf(move |path, data| {
                path == &expected_file_path && data == &expected_compressed_data_file
            })
            .returning(|_, _| Ok(()))
            .times(1);

        let obj = MockGitObject {
            compressor: Box::new(mock_compressor),
            file_utils: Box::new(mock_file_utils),
        };

        let result = obj.compress_and_write_to_disk(base_path.path());

        assert!(result.is_ok());
    }
}
