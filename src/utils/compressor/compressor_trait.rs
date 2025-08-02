#[cfg(test)]
use mockall::automock;


#[cfg_attr(test, automock)]
pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error>;
}
