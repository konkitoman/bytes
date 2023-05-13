use std::sync::Arc;

use crate::{TBuffer, TBytes};

impl<T: TBytes> TBytes for ::std::sync::Arc<T> {
    fn size(&self) -> usize {
        self.as_ref().size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.as_ref().to_bytes()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Arc::new(T::from_bytes(buffer)?))
    }
}
