#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "std")]
pub mod std;

pub type TBuffer<'a> = Vec<u8>;

pub trait TBytes {
    fn size(&self) -> usize;

    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized;

    fn from_bytes_ref(buffer: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let mut buffer = buffer.to_vec();
        let tmp = Self::from_bytes(&mut buffer);
        tmp
    }
}

pub extern crate bytes_kman_derive;
pub use bytes_kman_derive::Bytes;

pub mod prelude {
    pub use super::Bytes;
    pub use super::TBuffer;
    pub use super::TBytes;
}
