#[cfg(test)]
use mockall::automock;
use std::io;
use std::path::Path;

#[cfg_attr(test, automock)]
pub trait FileUtils {
    fn write_all(&self, path: &Path, data: &[u8]) -> io::Result<()>;
}
