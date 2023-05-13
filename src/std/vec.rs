use crate::{TBuffer, TBytes};

impl<T> TBytes for Vec<T>
where
    T: TBytes,
{
    fn size(&self) -> usize {
        let mut size = 0usize.size();
        for seg in self.iter() {
            size += seg.size()
        }
        size
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        buffer.append(&mut self.len().to_bytes());

        for seg in self.iter() {
            buffer.append(&mut seg.to_bytes())
        }

        buffer
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let len = usize::from_bytes(buffer)?;
        let mut res = Vec::with_capacity(len);

        for _ in 0..len {
            res.push(T::from_bytes(buffer)?)
        }

        Some(res)
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn vec_string() {
        let a = vec![String::from("Hello There!"), String::from("I am here!")];

        let mut b = a.to_bytes();

        let other = <Vec<String>>::from_bytes(&mut b.drain(..)).unwrap();

        assert_eq!(a, other)
    }
}
