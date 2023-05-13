use crate::{TBuffer, TBytes};

impl<T: TBytes> TBytes for std::sync::Mutex<T> {
    fn size(&self) -> usize {
        self.lock().unwrap().size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.lock().unwrap().to_bytes()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        Some(std::sync::Mutex::new(T::from_bytes(buffer)?))
    }
}
