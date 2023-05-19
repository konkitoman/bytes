use crate::{TBuffer, TBytes};

impl<T: TBytes> TBytes for Option<T> {
    fn size(&self) -> usize {
        match self {
            Some(d) => 1 + d.size(),
            None => 1,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        match self {
            Some(data) => {
                buffer.push(1);
                buffer.append(&mut data.to_bytes())
            }
            None => buffer.push(0),
        }

        buffer
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let mut iter = buffer.drain(0..Self::default().size());
        let has = iter.next()?;

        drop(iter);

        if has > 0 {
            Some(Some(T::from_bytes(buffer)?))
        } else {
            Some(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn option_string() {
        let a = Some(String::from("Hello There"));

        let mut bytes = a.to_bytes();

        let other = <Option<String>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other);

        let b: Option<String> = None;

        let mut bytes = b.to_bytes();

        let other = <Option<String>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(b, other)
    }
}
