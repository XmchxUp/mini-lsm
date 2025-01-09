mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

pub(crate) const SIZEOF_U16: usize = std::mem::size_of::<u16>();

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout illustrated in the tutorial
    /// Note: You may want to recheck if any of the expected field is missing from your output
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        let len = self.offsets.len();
        for ele in &self.offsets {
            buf.put_u16(*ele);
        }

        buf.put_u16(len as u16);
        buf.into()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        let offsets_len_start_idx = data.len() - SIZEOF_U16;
        let len = (&data[offsets_len_start_idx..]).get_u16() as usize;

        let data_end_idx = offsets_len_start_idx - len * SIZEOF_U16;
        let offsets_raw = &data[data_end_idx..offsets_len_start_idx];
        let offsets = offsets_raw
            .chunks(SIZEOF_U16)
            .map(|mut x| x.get_u16())
            .collect();

        let data = data[0..data_end_idx].to_vec();
        Self { data, offsets }
    }
}
