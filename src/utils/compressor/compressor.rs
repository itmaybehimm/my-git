use crate::utils::compressor::Compressor;
use flate2::{write::ZlibEncoder, Compression};
use std::io::Write;

pub struct ZlibCompressor;

impl Compressor for ZlibCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).expect("Failed to compress data");
        encoder.finish()
    }
}
