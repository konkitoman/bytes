use crate::TBytes;

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

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let start = T::from_bytes(buffer)?;
        let end = T::from_bytes(buffer)?;

        Some(start..end)
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

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let start = T::from_bytes(buffer)?;
        let end = T::from_bytes(buffer)?;

        Some(start..=end)
    }
}

impl<T: TBytes> TBytes for ::std::ops::RangeToInclusive<T> {
    fn size(&self) -> usize {
        self.end.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.end.to_bytes()
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
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

    fn from_bytes(_: &mut Vec<u8>) -> Option<Self>
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

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
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

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
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
        bytes.reverse();

        let b = <Range<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_to() {
        let a = ..21;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let b = <RangeTo<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_from() {
        let a = 21..;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let b = <RangeFrom<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_inclusive() {
        let a = 6..=21;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let b = <RangeInclusive<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }

    #[test]
    fn range_to_inclusive() {
        let a = ..=23;

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let b = <RangeToInclusive<i32>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b)
    }
}
