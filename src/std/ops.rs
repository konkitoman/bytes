use crate::{TBuffer, TBytes};

impl<T: TBytes> TBytes for ::std::ops::Range<T> {
    fn size(&self) -> usize {
        self.end.size() + self.end.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        buffer.append(&mut self.start.to_bytes());
        buffer.append(&mut self.end.to_bytes());

        buffer
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let start = T::from_bytes(buffer);
        let end = T::from_bytes(buffer);

        if let Some(start) = start {
            if let Some(end) = end {
                Some(start..end)
            } else {
                let mut bytes = start.to_bytes();
                while let Some(byte) = bytes.pop() {
                    buffer.insert(0, byte)
                }
                None
            }
        } else {
            None
        }
    }
}

impl<T: TBytes> TBytes for ::std::ops::RangeInclusive<T> {
    fn size(&self) -> usize {
        self.start().size() + self.end().size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        buffer.append(&mut self.start().to_bytes());
        buffer.append(&mut self.end().to_bytes());

        buffer
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let start = T::from_bytes(buffer);
        let end = T::from_bytes(buffer);

        if let Some(start) = start {
            if let Some(end) = end {
                Some(start..=end)
            } else {
                let mut bytes = start.to_bytes();
                while let Some(byte) = bytes.pop() {
                    buffer.insert(0, byte)
                }
                None
            }
        } else {
            None
        }
    }
}

impl<T: TBytes> TBytes for ::std::ops::RangeToInclusive<T> {
    fn size(&self) -> usize {
        self.end.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.end.to_bytes()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let end = T::from_bytes(buffer)?;
        Some(..=end)
    }
}

impl TBytes for ::std::ops::RangeFull {
    fn size(&self) -> usize {
        0
    }

    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }

    fn from_bytes(_: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        Some(..)
    }
}

impl<T: TBytes> TBytes for ::std::ops::RangeTo<T> {
    fn size(&self) -> usize {
        self.end.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.end.to_bytes()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let end = T::from_bytes(buffer)?;
        Some(..end)
    }
}

impl<T: TBytes> TBytes for ::std::ops::RangeFrom<T> {
    fn size(&self) -> usize {
        self.start.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.start.to_bytes()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let start = T::from_bytes(buffer)?;
        Some(start..)
    }
}

#[cfg(test)]
mod test {
    use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

    use crate::TBytes;

    #[test]
    fn range() {
        let a = 13..21;

        let mut bytes = a.to_bytes();

        let b = <Range<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_incomplete() {
        let mut buffer = Vec::new();
        buffer.append(&mut 50usize.to_bytes());

        let clone_buffer = buffer.clone();

        let other_buffer = Range::<usize>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.append(&mut 100usize.to_bytes());

        let value = Range::<usize>::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, 50..100)
    }

    #[test]
    fn range_to() {
        let a = ..21;

        let mut bytes = a.to_bytes();

        let b = <RangeTo<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_from() {
        let a = 21..;

        let mut bytes = a.to_bytes();

        let b = <RangeFrom<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_inclusive() {
        let a = 6..=21;

        let mut bytes = a.to_bytes();

        let b = <RangeInclusive<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_inclusive_incomplete() {
        let mut buffer = Vec::new();
        buffer.append(&mut 50usize.to_bytes());

        let clone_buffer = buffer.clone();

        let other_buffer = RangeInclusive::<usize>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.append(&mut 100usize.to_bytes());

        let value = RangeInclusive::<usize>::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, 50..=100)
    }

    #[test]
    fn range_to_inclusive() {
        let a = ..=23;

        let mut bytes = a.to_bytes();

        let b = <RangeToInclusive<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }
}
