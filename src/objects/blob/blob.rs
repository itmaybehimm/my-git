use crate::objects::traits::GitObject;
use crate::objects::types::ObjectType;
use crate::utils::{Compressor, FileUtils};

pub struct Blob {
    pub content: Vec<u8>,
    pub compressor: Box<dyn Compressor>,
    pub file_utils: Box<dyn FileUtils>,
}

impl GitObject for Blob {
    fn get_object_type(&self) -> &'static str {
        ObjectType::BLOB.as_str()
    }

    fn get_content_bytes(&self) -> Vec<u8> {
        self.content.clone()
    }

    fn get_compressor(&self) -> &dyn Compressor {
        self.compressor.as_ref()
    }

    fn get_file_utils(&self) -> &dyn FileUtils {
        self.file_utils.as_ref()
    }
}
