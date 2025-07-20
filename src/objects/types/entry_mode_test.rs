#[cfg(test)]
mod tests {
    use super::super::entry_mode::*;

    #[test]
    fn test_as_str() {
        assert_eq!(EntryMode::File.as_str(), "100644");
        assert_eq!(EntryMode::Directory.as_str(), "040000");
    }
}
