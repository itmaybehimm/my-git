#[cfg(test)]
mod tests {
    use super::super::blob::*;
    use crate::objects::traits::GitObject;
    use crate::objects::types::ObjectType;
    use crate::utils::{Compressor, FileUtils, MockCompressor, MockFileUtils};

    #[test]
    fn test_get_object_type() {
        let blob = Blob {
            content: b"blob".to_vec(),
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };

        assert_eq!(blob.get_object_type(), ObjectType::BLOB.as_str());
    }

    #[test]
    fn test_get_content_bytes() {
        let content = b"fn main() {\n    println!(\"Hello, Git!\");\n}".to_vec();
        let blob = Blob {
            content: content.clone(),
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };

        assert_eq!(blob.get_content_bytes(), content);
    }

    #[test]
    fn test_get_compressor() {
        let mockCompressor = Box::new(MockCompressor::new());
        let ptr = mockCompressor.as_ref() as *const dyn Compressor;

        let blob = Blob {
            content: b"blob".to_vec(),
            compressor: mockCompressor,
            file_utils: Box::new(MockFileUtils::new()),
        };

        let returned = blob.get_compressor() as *const dyn Compressor;

        assert_eq!(ptr, returned);
    }

    #[test]
    fn test_get_file_utils() {
        let mock_file_utils = Box::new(MockFileUtils::new());
        let ptr = mock_file_utils.as_ref() as *const dyn FileUtils;

        let blob = Blob {
            content: b"blob".to_vec(),
            compressor: Box::new(MockCompressor::new()),
            file_utils: mock_file_utils,
        };

        let returned = blob.get_file_utils() as *const dyn FileUtils;

        assert_eq!(ptr, returned);
    }
}
