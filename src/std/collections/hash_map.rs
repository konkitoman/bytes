use ::std::collections::hash_map::HashMap;

use crate::{TBuffer, TBytes};

impl<T, A> TBytes for HashMap<T, A>
where
    T: TBytes + Eq + ::std::hash::Hash,
    A: TBytes,
{
    fn size(&self) -> usize {
        let mut size = 0usize.size();

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

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::default().size() {
            return None;
        }
        let len = usize::from_bytes(buffer)?;

        let mut res = Vec::new();

        for _ in 0..len {
            let key = T::from_bytes(buffer);
            let value = A::from_bytes(buffer);
            if let Some(key) = key {
                if let Some(value) = value {
                    res.push((key, value));
                } else {
                    let mut bytes = key.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                    while let Some(element) = res.pop() {
                        let mut bytes = element.1.to_bytes();
                        while let Some(byte) = bytes.pop() {
                            buffer.insert(0, byte)
                        }
                        let mut bytes = element.0.to_bytes();
                        while let Some(byte) = bytes.pop() {
                            buffer.insert(0, byte)
                        }
                    }
                    let mut bytes = len.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                    return None;
                }
            } else {
                while let Some(element) = res.pop() {
                    let mut bytes = element.1.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                    let mut bytes = element.0.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                }
                let mut bytes = len.to_bytes();
                while let Some(byte) = bytes.pop() {
                    buffer.insert(0, byte)
                }
                return None;
            }
        }

        let mut hashmap = HashMap::new();

        for (key, value) in res {
            hashmap.insert(key, value);
        }

        Some(hashmap)
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

        let other = <HashMap<String, String>>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn incomplete() {
        let mut buffer = Vec::new();
        buffer.append(&mut 2usize.to_bytes());
        buffer.append(&mut String::from("Foo").to_bytes());
        buffer.append(&mut String::from("Bar").to_bytes());

        let clone_buffer = buffer.clone();
        let other_buffer = HashMap::<String, String>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.append(&mut String::from("James").to_bytes());
        buffer.append(&mut String::from("21").to_bytes());

        let value = HashMap::<String, String>::from_bytes(&mut buffer).unwrap();
        let mut hashmap = HashMap::<String, String>::new();
        hashmap.insert("Foo".into(), "Bar".into());
        hashmap.insert("James".into(), "21".into());
        assert_eq!(value, hashmap)
    }
}
