use crate::{TBuffer, TBytes};

pub mod num;
pub mod option;
pub mod primitive;
pub mod result;
pub mod tuple;

impl<T: TBytes, const LEN: usize> TBytes for [T; LEN] {
    fn size(&self) -> usize {
        let mut size = 0;
        for p in self.iter() {
            size += p.size();
        }
        size
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());

        for p in self.iter() {
            buffer.append(&mut p.to_bytes())
        }

        buffer
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let mut res = Vec::with_capacity(LEN);

        for _ in 0..LEN {
            if let Some(value) = T::from_bytes(buffer) {
                res.push(value)
            } else {
                while let Some(element) = res.pop() {
                    let mut bytes = element.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                }
                return None;
            }
        }

        // this will create a static reference
        // because of that will not be droped
        // if this was as_ref on this scope will have tow variabiles with the same memory and at
        // the end of the scope will be droped and that will create a dangling pointer
        let res = Box::leak(res.into_boxed_slice());
        // this will read the memory will not take
        // in this scope only one variabile has access to the memory
        // and that variabile is moved out
        // so only if that variabile will be droped the memory will be freed
        let res = unsafe { std::ptr::read(res.as_ptr() as *const [T; LEN]) };

        Some(res)
    }
}

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn slice_i32() {
        let a = [32, 543, 61, 21215, -4236, 32];

        let mut bytes = a.to_bytes();

        let other = <[i32; 6]>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other);
    }

    #[test]
    fn slice_string() {
        let a = [
            "Hello World!".to_string(),
            "This is working???".into(),
            "Is working as is supposed!".into(),
        ];

        let mut bytes = a.to_bytes();

        let other = <[String; 3]>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other);
    }

    #[test]
    fn incomplete() {
        let mut buffer = Vec::new();
        let strings = ["Where", "Are", "You", "From"].map(|w| w.to_string());
        for string in &strings[0..3] {
            buffer.append(&mut string.to_bytes());
        }
        let clone_buffer = buffer.clone();

        let other_buffer = <[String; 4]>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.append(&mut strings[3].to_bytes());

        let value = <[String; 4]>::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, strings)
    }
}
