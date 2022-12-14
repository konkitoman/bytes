use ::std::collections::hash_map::HashMap;

use crate::TBytes;

impl<T, A> TBytes for HashMap<T, A>
where
    T: TBytes + Eq + ::std::hash::Hash,
    A: TBytes,
{
    fn size(&self) -> usize {
        let mut size = 0;

        for (key, value) in self.iter() {
            size += key.size();
            size += value.size();
        }

        size
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        buffer.append(&mut self.len().to_bytes());

        for (key, value) in self.iter() {
            buffer.append(&mut key.to_bytes());
            buffer.append(&mut value.to_bytes());
        }

        buffer
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let len = usize::from_bytes(buffer)?;

        let mut res = HashMap::new();

        for _ in 0..len {
            let key = T::from_bytes(buffer)?;
            let value = A::from_bytes(buffer)?;
            res.insert(key, value);
        }

        Some(res)
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;
    use std::collections::HashMap;

    #[test]
    fn hash_map() {
        let mut a = HashMap::new();
        a.insert("QQQL".to_string(), "Maniac".to_string());
        a.insert("Content-Length".to_string(), "21".to_string());

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let other = <HashMap<String, String>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other)
    }
}
