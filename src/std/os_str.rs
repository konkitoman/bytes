use std::os::unix::prelude::{OsStrExt, OsStringExt};

use crate::TBytes;

impl TBytes for std::ffi::OsString {
    fn size(&self) -> usize {
        self.len() + 0usize.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());
        buffer.append(&mut self.len().to_bytes());
        for byte in self.as_bytes() {
            buffer.push(*byte)
        }
        buffer
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let len = usize::from_bytes(buffer)?;
        let mut buff = Vec::with_capacity(len);
        for _ in 0..len {
            buff.push(buffer.pop()?)
        }
        Some(std::ffi::OsString::from_vec(buff))
    }
}
