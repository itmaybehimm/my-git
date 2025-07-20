use crate::objects::tree::TreeEntry;
use crate::objects::types::EntryMode;
use crate::utils::get_sha1_hash;

pub fn mock_tree_entry(
    mode: Option<EntryMode>,
    name: Option<&str>,
    hash: Option<&[u8; 20]>,
) -> TreeEntry {
    let mode = mode.unwrap_or(EntryMode::File);
    let name = name.unwrap_or("hello.txt").to_string();
    let hash = hash.unwrap_or(&get_sha1_hash(&"test".as_bytes().to_vec())).clone();

    TreeEntry { mode, name, hash }
}
