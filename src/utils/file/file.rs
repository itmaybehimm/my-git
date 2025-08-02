use crate::utils::FileUtils;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::Path;

pub struct DiskFileUtils;

impl FileUtils for DiskFileUtils {
    fn write_all(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        file.write_all(data)
    }
}
