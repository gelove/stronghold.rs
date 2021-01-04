mod decoder;
mod encoder;

pub use decoder::decompress;
pub use encoder::compress;
#[derive(Debug)]
pub(crate) struct Block {
    literal_length: usize,
    duplicates: Option<Duplicate>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Duplicate {
    offset: u16,
    padding: usize,
}
