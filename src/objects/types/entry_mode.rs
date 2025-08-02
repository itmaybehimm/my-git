#[derive(Debug, PartialEq)]
pub enum EntryMode {
    File,
    Directory,
}

impl EntryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::File => "100644",
            Self::Directory => "040000",
        }
    }
}
