#[cfg(test)]
mod tests {
    use super::super::tree::*;
    use crate::__mocks__::mock_tree_entry;
    use crate::objects::traits::GitObject;
    use crate::objects::types::{EntryMode, ObjectType};
    use crate::utils::{Compressor, FileUtils, MockCompressor, MockFileUtils};

    fn mock_file_entry() -> TreeEntry {
        mock_tree_entry(Some(EntryMode::File), Some("hello.txt"), Some(&[0xab; 20]))
    }
    fn mock_dir_entry() -> TreeEntry {
        mock_tree_entry(Some(EntryMode::Directory), Some("src"), Some(&[0xff; 20]))
    }

    #[test]
    fn test_get_object_type() {
        let tree = Tree {
            entries: vec![],
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };
        assert_eq!(tree.get_object_type(), ObjectType::TREE.as_str());
    }

    #[test]
    fn test_content_bytes() {
        let tree = Tree {
            entries: vec![mock_file_entry(), mock_dir_entry()],
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };
        let content = tree.get_content_bytes();

        let mut expected = Vec::new();
        expected.extend(b"100644 hello.txt\0");
        expected.extend(&[0xab; 20]);
        expected.extend(b"040000 src\0");
        expected.extend(&[0xff; 20]);

        assert_eq!(content, expected);
    }

    #[test]
    fn test_add_entry() {
        let mut tree = Tree {
            entries: vec![],
            compressor: Box::new(MockCompressor::new()),
            file_utils: Box::new(MockFileUtils::new()),
        };

        tree.add_entry(mock_file_entry());

        let file_entry = mock_file_entry();

        assert_eq!(tree.entries.len(), 1);

        let entry = &tree.entries[0];

        assert_eq!(entry.mode, file_entry.mode);
        assert_eq!(entry.name, file_entry.name);
        assert_eq!(entry.hash, file_entry.hash);
    }

    #[test]
    fn test_get_compressor() {
        let mockCompressor = Box::new(MockCompressor::new());
        let ptr = mockCompressor.as_ref() as *const dyn Compressor;

        let tree = Tree {
            entries: vec![],
            compressor: mockCompressor,
            file_utils: Box::new(MockFileUtils::new()),
        };

        let returned = tree.get_compressor() as *const dyn Compressor;

        assert_eq!(ptr, returned);
    }

    #[test]
    fn test_get_file_utils() {
        let mock_file_utils = Box::new(MockFileUtils::new());
        let ptr = mock_file_utils.as_ref() as *const dyn FileUtils;

        let tree = Tree {
            entries: vec![],
            compressor: Box::new(MockCompressor::new()),
            file_utils: mock_file_utils,
        };

        let returned = tree.get_file_utils() as *const dyn FileUtils;

        assert_eq!(ptr, returned);
    }
}
