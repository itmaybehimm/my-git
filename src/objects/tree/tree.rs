use crate::objects::traits::GitObject;
use crate::objects::types::{EntryMode, ObjectType};
use crate::utils::{Compressor, FileUtils};

pub struct TreeEntry {
    pub mode: EntryMode,
    pub name: String,
    pub hash: [u8; 20],
}

pub struct Tree {
    pub entries: Vec<TreeEntry>,
    pub compressor: Box<dyn Compressor>,
    pub file_utils: Box<dyn FileUtils>,
}

impl GitObject for Tree {
    fn get_object_type(&self) -> &'static str {
        ObjectType::TREE.as_str()
    }

    fn get_content_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        for entry in &self.entries {
            let header = format!("{} {}\0", entry.mode.as_str(), entry.name);
            out.extend(header.as_bytes());
            out.extend(&entry.hash);
        }

        out
    }

    fn get_compressor(&self) -> &dyn Compressor {
        self.compressor.as_ref()
    }

    fn get_file_utils(&self) -> &dyn FileUtils {
        self.file_utils.as_ref()
    }
}

impl Tree {
    pub fn add_entry(&mut self, entry: TreeEntry) {
        self.entries.push(entry);
    }
}
