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
}
