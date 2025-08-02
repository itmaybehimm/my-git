use crate::objects::traits::GitObject;
use crate::objects::types::ObjectType;
use hex::encode;
pub struct Commit {
    pub tree: [u8; 20],
    pub parents: Vec<[u8; 20]>,
    pub author: String,
    pub author_email: String,
    pub author_time: i64,
    pub author_tz: String,
    pub committer: String,
    pub committer_email: String,
    pub committer_time: i64,
    pub committer_tz: String,
    pub message: String,
}

impl GitObject for Commit {
    fn get_object_type(&self) -> &'static str {
        ObjectType::COMMIT.as_str()
    }

    fn get_content_bytes(&self) -> Vec<u8> {
        let mut content = Vec::new();
        content.extend(format!("tree {}\n", encode(self.tree)).as_bytes());
        for parent in &self.parents {
            content.extend(format!("parent {}\n", encode(parent)).as_bytes());
        }
        content.extend(
            format!(
                "author {} <{}> {} {}\n",
                self.author, self.author_email, self.author_time, self.author_tz
            )
            .as_bytes(),
        );
        content.extend(
            format!(
                "committer {} <{}> {} {}\n",
                self.committer, self.committer_email, self.committer_time, self.committer_tz
            )
            .as_bytes(),
        );

        content.push(b'\n');

        content.extend(self.message.as_bytes());

        content
    }
}
