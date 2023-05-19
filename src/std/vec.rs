use crate::{TBuffer, TBytes};

impl<T: TBytes> TBytes for Vec<T> {
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
        if buffer.len() < 0usize.size() {
            return None;
        }
        let len = usize::from_bytes(buffer)?;
        let mut res = Vec::with_capacity(len);

        for _ in 0..len {
            if let Some(element) = T::from_bytes(buffer) {
                res.push(element)
            } else {
                while let Some(element) = res.pop() {
                    let mut bytes = element.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte);
                    }
                }
                let mut bytes = len.to_bytes();
                while let Some(byte) = bytes.pop() {
                    buffer.insert(0, byte);
                }
                return None;
            }
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

        let other = <Vec<String>>::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }

    #[test]
    fn incoplete() {
        let mut buffer = Vec::new();
        buffer.append(&mut 4usize.to_bytes());
        buffer.push(21);
        buffer.push(69);
        let clone_buffer = buffer.clone();

        let other_buffer = Vec::<u8>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer)
    }

    #[test]
    fn incoplete_unsized() {
        let mut buffer = Vec::new();
        buffer.append(&mut 4usize.to_bytes());
        buffer.append(&mut String::from("Hello World!").to_bytes());
        buffer.append(&mut String::from("Hello Made!").to_bytes());
        buffer.append(&mut String::from("Samuel?").to_bytes());
        let clone_buffer = buffer.clone();

        let other_buffer = Vec::<String>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer)
    }
}
