#[cfg(test)]
mod tests {
    use super::super::tree::*;
    use crate::__mocks__::mock_hash::mock_hash;
    use crate::__mocks__::mock_tree_entry::mock_tree_entry;
    use crate::objects::traits::GitObject;
    use crate::objects::types::{EntryMode, ObjectType};
    use crate::utils::hasher::hasher::get_sha1_hash;

    fn mock_file_entry() -> TreeEntry {
        mock_tree_entry(
            Some(EntryMode::File),
            Some("hello.txt"),
            Some(&mock_hash(Some("file".to_string()))),
        )
    }
    fn mock_dir_entry() -> TreeEntry {
        mock_tree_entry(
            Some(EntryMode::File),
            Some("src"),
            Some(&mock_hash(Some("directory".to_string()))),
        )
    }

    #[test]
    fn test_get_object_type() {
        let tree = Tree::new(vec![]);
        assert_eq!(tree.get_object_type(), ObjectType::TREE.as_str());
    }

    #[test]
    fn test_content_bytes() {
        let tree = Tree::new(vec![mock_file_entry(), mock_dir_entry()]);
        let content = tree.get_content_bytes();

        let mut expected = Vec::new();
        expected.extend(b"100644 hello.txt\0");
        expected.extend(get_sha1_hash(&"file".as_bytes().to_vec()));
        expected.extend(b"040000 src\0");
        expected.extend(get_sha1_hash(&"directory".as_bytes().to_vec()));

        assert_eq!(content, expected);
    }

    #[test]
    fn test_add_entry() {
        let mut tree = Tree::new(vec![]);

        tree.add_entry(mock_file_entry());

        let file_entry = mock_file_entry();

        assert_eq!(tree.get_entries().len(), 1);

        let entry = &tree.get_entries()[0];

        assert_eq!(entry.mode, file_entry.mode);
        assert_eq!(entry.name, file_entry.name);
        assert_eq!(entry.hash, file_entry.hash);
    }

    #[test]
    fn test_get_entries() {
        let tree = Tree::new(vec![mock_file_entry()]);
        assert_eq!(tree.get_entries().len(), 1);
        let entry = &tree.get_entries()[0];
        assert_eq!(entry.name, "hello.txt");
    }
}
