#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "std")]
pub mod std;

pub trait TBytes {
    fn size(&self) -> usize;

    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized;
}

#[macro_use]
pub extern crate bytes_kman_derive;
pub use bytes_kman_derive::Bytes;

pub mod prelude {
    pub use super::Bytes;
    pub use super::TBytes;
}
