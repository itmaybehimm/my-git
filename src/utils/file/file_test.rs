#[cfg(test)]
mod tests {
    use super::super::*;
    use std::fs;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_write_all() {
        let dir = tempdir().expect("Failed to create temp dir");

        let file_path = dir.path().join("subdir").join("testfile.txt");

        let data = b"Hello, temp file!";

        let file_utils = DiskFileUtils;
        file_utils
            .write_all(&file_path, data)
            .expect("write_all failed");

        let mut file_content = Vec::new();
        let mut file = fs::File::open(&file_path).expect("Failed to open written file");
        file.read_to_end(&mut file_content)
            .expect("Failed to read file");

        assert_eq!(file_content, data);
    }
}
