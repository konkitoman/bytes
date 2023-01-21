use crate::TBytes;

impl<T: TBytes, E: TBytes> TBytes for Result<T, E> {
    fn size(&self) -> usize {
        match self {
            Ok(res) => res.size() + 1,
            Err(err) => err.size() + 1,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        match self {
            Ok(res) => {
                buffer.push(0);
                buffer.append(&mut res.to_bytes())
            }
            Err(err) => {
                buffer.push(1);
                buffer.append(&mut err.to_bytes())
            }
        }

        buffer
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let variant = buffer.pop()?;

        match variant {
            0 => Some(Ok(T::from_bytes(buffer)?)),
            1 => Some(Err(E::from_bytes(buffer)?)),
            _ => None,
        }
    }
}

impl TBytes for () {
    fn size(&self) -> usize {
        0
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(_buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(())
    }
}
