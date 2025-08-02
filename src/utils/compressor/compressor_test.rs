#[cfg(test)]
mod tests {
    use super::super::*;
    use flate2::{write::ZlibEncoder, Compression};
    use std::io::Write;

    #[test]
    fn test_compress() {
        let compressor = ZlibCompressor;
        let input = b"some test data for compression";

        let compressed = compressor.compress(input).unwrap();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(input).unwrap();
        let result = encoder.finish().unwrap();

        assert_eq!(compressed, result);
    }
}
