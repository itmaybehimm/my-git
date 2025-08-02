mod compressor;
mod file;
pub mod hasher;

pub use compressor::Compressor;
pub use file::FileUtils;
pub use hasher::get_sha1_hash;

#[cfg(test)]
pub use compressor::MockCompressor;

#[cfg(test)]
pub use file::MockFileUtils;
