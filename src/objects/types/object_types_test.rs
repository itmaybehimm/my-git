#[cfg(test)]
mod tests {
    use super::super::object_types::*;
    #[test]
    fn test_object_type_as_str() {
        assert_eq!(ObjectType::BLOB.as_str(), "blob");
        assert_eq!(ObjectType::TREE.as_str(), "tree");
        assert_eq!(ObjectType::COMMIT.as_str(), "commit");
    }
}
