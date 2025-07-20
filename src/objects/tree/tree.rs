use crate::objects::traits::GitObject;
use crate::objects::types::{EntryMode, ObjectType};

pub struct TreeEntry {
    pub mode: EntryMode,
    pub name: String,
    pub hash: [u8; 20],
}

pub struct Tree {
    entries: Vec<TreeEntry>,
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
}

impl Tree {
    pub fn new(entries: Vec<TreeEntry>) -> Self {
        Tree { entries }
    }

    pub fn add_entry(&mut self, entry: TreeEntry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &[TreeEntry] {
        &self.entries
    }
}
