use crate::TBytes;

impl<T: TBytes> TBytes for std::sync::RwLock<T> {
    fn size(&self) -> usize {
        self.read().unwrap().size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.read().unwrap().to_bytes()
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(std::sync::RwLock::new(T::from_bytes(buffer)?))
    }
}
