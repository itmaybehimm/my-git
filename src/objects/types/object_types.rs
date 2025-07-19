#[derive(Debug, PartialEq)]
pub enum ObjectType {
    BLOB,
    TREE,
    COMMIT,
}

impl ObjectType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            ObjectType::COMMIT => "commit",
            ObjectType::TREE => "tree",
            ObjectType::BLOB => "blob",
        }
    }
}
