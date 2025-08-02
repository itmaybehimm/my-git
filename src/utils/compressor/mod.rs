mod compressor;
mod compressor_test;
mod compressor_trait;

pub use compressor::ZlibCompressor;
pub use compressor_trait::Compressor;

#[cfg(test)]
pub use compressor_trait::MockCompressor;
