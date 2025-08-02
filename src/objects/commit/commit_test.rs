#[cfg(test)]
mod tests {
    use super::super::commit::*;
    use crate::objects::traits::GitObject;
    use crate::objects::types::ObjectType;
    use crate::utils::{Compressor, FileUtils, MockCompressor, MockFileUtils};
    use hex::encode;

    fn mock_commit(
        compressor: Option<Box<dyn Compressor>>,
        file_utils: Option<Box<dyn FileUtils>>,
    ) -> Commit {
        let tree_hash = [0xab; 20];
        let parent_hash = [0xcd; 20];
        Commit {
            tree: tree_hash,
            parents: vec![parent_hash],
            author: "Jane Doe".into(),
            author_email: "jane@example.com".into(),
            author_time: 1687804800,
            author_tz: "+0000".into(),
            committer: "Jane Doe".into(),
            committer_email: "jane@example.com".into(),
            committer_time: 1687804800,
            committer_tz: "+0000".into(),
            message: "Initial commit".into(),

            compressor: compressor.unwrap_or_else(|| Box::new(MockCompressor::new())),
            file_utils: file_utils.unwrap_or_else(|| Box::new(MockFileUtils::new())),
        }
    }

    #[test]
    fn test_get_object_type() {
        let commit = mock_commit(None, None);
        assert_eq!(commit.get_object_type(), ObjectType::COMMIT.as_str());
    }

    #[test]
    fn test_get_content_bytes() {
        let commit = mock_commit(None, None);

        let content_bytes = commit.get_content_bytes();
        let content_str = String::from_utf8_lossy(&content_bytes);

        let expected = format!(
            "tree {}\nparent {}\nauthor Jane Doe <jane@example.com> 1687804800 +0000\ncommitter Jane Doe <jane@example.com> 1687804800 +0000\n\nInitial commit",
            encode([0xab; 20]),
            encode([0xcd; 20])
        );

        assert_eq!(content_str, expected);
    }

    #[test]
    fn test_get_compressor() {
        let mock_compressor = Box::new(MockCompressor::new());
        let ptr = mock_compressor.as_ref() as *const dyn Compressor;

        let commit = mock_commit(Some(mock_compressor), None);

        let returned = commit.get_compressor() as *const dyn Compressor;

        assert_eq!(ptr, returned);
    }

    #[test]
    fn test_get_file_utils() {
        let mock_file_utils = Box::new(MockFileUtils::new());
        let ptr = mock_file_utils.as_ref() as *const dyn FileUtils;

        let commit = mock_commit(None, Some(mock_file_utils));

        let returned = commit.get_file_utils() as *const dyn FileUtils;

        assert_eq!(ptr, returned);
    }
}
