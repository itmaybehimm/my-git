#[cfg(test)]
mod tests {
    use super::super::blob::*;
    use crate::objects::traits::GitObject;
    use crate::objects::types::ObjectType;

    #[test]
    fn test_get_object_type() {
        let blob = Blob::new(b"blob".to_vec());

        assert_eq!(blob.get_object_type(), ObjectType::BLOB.as_str());
    }

    #[test]
    fn test_get_content_bytes() {
        let content = b"fn main() {\n    println!(\"Hello, Git!\");\n}".to_vec();
        let blob = Blob::new(content.clone());

        assert_eq!(blob.get_content_bytes(), content);
    }
}
