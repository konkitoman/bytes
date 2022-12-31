use crate::TBytes;

pub mod num;
pub mod option;
pub mod primitive;

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

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut res = Vec::with_capacity(LEN);

        for _ in 0..LEN {
            res.push(T::from_bytes(buffer)?)
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
        bytes.reverse();

        let other = <[i32; 6]>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other);
    }

    #[test]
    fn slice_string() {
        let a = [
            format!("Hello World!"),
            format!("This is working???"),
            format!("Is working as is supposed!"),
        ];

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let other = <[String; 3]>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, other);
    }
}
