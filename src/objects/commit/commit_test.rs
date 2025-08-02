#[cfg(test)]
mod tests {
    use super::super::commit::*;
    use crate::objects::traits::GitObject;
    use crate::objects::types::ObjectType;
    use hex::encode;

    fn mock_commit() -> Commit {
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
        }
    }
    #[test]
    fn test_get_object_type() {
        let commit = mock_commit();
        assert_eq!(commit.get_object_type(), ObjectType::COMMIT.as_str());
    }

    #[test]
    fn test_get_content_bytes() {
        let commit = mock_commit();

        let content_bytes = commit.get_content_bytes();
        let content_str = String::from_utf8_lossy(&content_bytes);

        let expected = format!(
            "tree {}\nparent {}\nauthor Jane Doe <jane@example.com> 1687804800 +0000\ncommitter Jane Doe <jane@example.com> 1687804800 +0000\n\nInitial commit",
            encode([0xab; 20]),
            encode([0xcd; 20])
        );

        assert_eq!(content_str, expected);
    }
}
