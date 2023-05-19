use crate::{TBuffer, TBytes};
use ::core::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

impl TBytes for u8 {
    fn size(&self) -> usize {
        1
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self> {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [buffer.next()?];
        Some(u8::from_le_bytes(bytes))
    }
}

impl TBytes for u16 {
    fn size(&self) -> usize {
        2
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self> {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [buffer.next()?, buffer.next()?];
        Some(u16::from_le_bytes(bytes))
    }
}

impl TBytes for u32 {
    fn size(&self) -> usize {
        4
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(u32::from_le_bytes(bytes))
    }
}

impl TBytes for u64 {
    fn size(&self) -> usize {
        8
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(u64::from_le_bytes(bytes))
    }
}

impl TBytes for u128 {
    fn size(&self) -> usize {
        16
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(u128::from_le_bytes(bytes))
    }
}

impl TBytes for usize {
    fn size(&self) -> usize {
        8
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(usize::from_le_bytes(bytes))
    }
}

impl TBytes for i8 {
    fn size(&self) -> usize {
        1
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self> {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [buffer.next()?];
        Some(i8::from_le_bytes(bytes))
    }
}

impl TBytes for i16 {
    fn size(&self) -> usize {
        2
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self> {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [buffer.next()?, buffer.next()?];
        Some(i16::from_le_bytes(bytes))
    }
}

impl TBytes for i32 {
    fn size(&self) -> usize {
        4
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(i32::from_le_bytes(bytes))
    }
}

impl TBytes for i64 {
    fn size(&self) -> usize {
        8
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(i64::from_le_bytes(bytes))
    }
}

impl TBytes for i128 {
    fn size(&self) -> usize {
        16
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(i128::from_le_bytes(bytes))
    }
}

impl TBytes for isize {
    fn size(&self) -> usize {
        8
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut buffer = buffer.drain(0..Self::default().size());
        let bytes = [
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
            buffer.next()?,
        ];
        Some(isize::from_le_bytes(bytes))
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn u8() {
        let a = 42u8;

        let mut b = a.to_bytes();
        let other = u8::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn u16() {
        let a = 42u16;

        let mut b = a.to_bytes();
        let other = u16::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn u32() {
        let a = 42u32;

        let mut b = a.to_bytes();
        let other = u32::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn u64() {
        let a = 42u64;

        let mut b = a.to_bytes();
        let other = u64::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn u128() {
        let a = 42u128;

        let mut b = a.to_bytes();
        let other = u128::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn usize() {
        let a = 42usize;

        let mut b = a.to_bytes();
        let other = usize::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn i8() {
        let a = 42i8;

        let mut b = a.to_bytes();
        let other = i8::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn i16() {
        let a = 42i16;

        let mut b = a.to_bytes();
        let other = i16::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn i32() {
        let a = 42i32;

        let mut b = a.to_bytes();
        let other = i32::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn i64() {
        let a = 42i64;

        let mut b = a.to_bytes();
        let other = i64::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn i128() {
        let a = 42i128;

        let mut b = a.to_bytes();
        let other = i128::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn isize() {
        let a = 42isize;

        let mut b = a.to_bytes();
        let other = isize::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn incomplete_u16() {
        let mut buffer = Vec::new();
        buffer.push(10);
        let clone_buffer = buffer.clone();

        let other_buffer = u16::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.push(1);

        let value = u16::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, 266)
    }
}
