use crate::objects::traits::GitObject;
use crate::objects::types::ObjectType;

pub struct Blob {
    object_type: ObjectType,
    pub context: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self {
            object_type: ObjectType::BLOB,
            context: content,
        }
    }
}

impl GitObject for Blob {
    fn object_type(&self) -> &'static str {
        self.object_type.as_str()
    }

    fn content_bytes(&self) -> Vec<u8> {
        self.context.clone()
    }
}
