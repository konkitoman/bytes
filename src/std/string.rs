use crate::{TBuffer, TBytes};

use std::string::String;

impl TBytes for String {
    fn size(&self) -> usize {
        self.len() + 0usize.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        buffer.append(&mut self.len().to_bytes());
        buffer.append(&mut self.as_bytes().to_vec());

        buffer
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < 0usize.size() {
            return None;
        }
        let len = usize::from_bytes(buffer)?;
        if buffer.len() < len {
            let mut bytes = len.to_bytes();
            while let Some(byte) = bytes.pop() {
                buffer.insert(0, byte);
            }
            return None;
        }

        let mut res = String::with_capacity(len);
        let mut iter = buffer.drain(..len);
        for _ in 0..len {
            res.push(iter.next()? as char);
        }
        drop(iter);
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn string() {
        let a = String::from("Hello World!!!\nHow!!!");

        let mut b = a.to_bytes();

        let other = String::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn incomplete() {
        let mut buffer = Vec::new();
        buffer.append(&mut 4usize.to_bytes());
        buffer.push(b'g');
        buffer.push(b'r');
        buffer.push(b'a');
        let clone_buffer = buffer.clone();

        let other_buffer = String::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.push(b'y');

        let value = String::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, String::from("gray"))
    }
}
