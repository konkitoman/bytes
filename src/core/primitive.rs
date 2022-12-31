use ::core::primitive::bool;

use crate::TBytes;

impl TBytes for bool {
    fn size(&self) -> usize {
        1
    }

    fn to_bytes(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let byte = buffer.pop()?;
        if byte > 0 {
            Some(true)
        } else {
            Some(false)
        }
    }
}

impl TBytes for f32 {
    fn size(&self) -> usize {
        4
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self::from_le_bytes([
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
        ]))
    }
}

impl TBytes for f64 {
    fn size(&self) -> usize {
        8
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self::from_le_bytes([
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
            buffer.pop()?,
        ]))
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn bool() {
        let a = true;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let other = bool::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other);

        let b = true;

        let mut bytes = b.to_bytes();
        bytes.reverse();

        let other = bool::from_bytes(&mut bytes).unwrap();

        assert_eq!(b, other)
    }

    #[test]
    fn f32() {
        let a = 5234.0f32;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let other = f32::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn f64() {
        let a = 43223.32f64;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let other = f64::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other)
    }
}
