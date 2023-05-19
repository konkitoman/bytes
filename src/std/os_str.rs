use std::ffi::OsString;

use crate::{TBuffer, TBytes};

// TODO: Should fix on linux utf16 characters will be ignored
impl TBytes for std::ffi::OsString {
    fn size(&self) -> usize {
        self.len() * 2 + 0usize.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size());
        buffer.append(&mut self.len().to_bytes());
        #[cfg(target_os = "linux")]
        use std::os::unix::ffi::OsStrExt;
        #[cfg(target_os = "windows")]
        use std::os::windows::ffi::OsStrExt;

        #[cfg(target_os = "linux")]
        for byte in self.as_bytes() {
            {
                buffer.push(*byte);
                buffer.push(0);
            }
        }
        #[cfg(target_os = "windows")]
        for byte in self.encode_wide() {
            buffer.append(&mut byte.to_bytes())
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
        if buffer.len() < len * 2 {
            let mut bytes = len.to_bytes();
            while let Some(byte) = bytes.pop() {
                buffer.insert(0, byte)
            }
            return None;
        }

        let mut buff = Vec::with_capacity(len);
        for _ in 0..len {
            #[cfg(target_os = "linux")]
            {
                let mut iter = buffer.drain(..2);
                let value = iter.next();
                if let Some(value) = value {
                    if let Some(_) = iter.next() {
                        buff.push(value);
                    } else {
                        drop(iter);
                        buffer.insert(0, value);
                        while let Some(byte) = buff.pop() {
                            buffer.insert(0, byte);
                        }
                        let mut bytes = len.to_bytes();
                        while let Some(byte) = bytes.pop() {
                            buffer.insert(0, byte)
                        }
                        return None;
                    }
                } else {
                    drop(iter);
                    while let Some(byte) = buff.pop() {
                        buffer.insert(0, byte);
                    }
                    let mut bytes = len.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                    return None;
                }
            }
            #[cfg(target_os = "windows")]
            {
                if let Some(value) = u16::from_bytes(buffer) {
                    buff.push(value);
                } else {
                    while let Some(element) = buff.pop() {
                        let mut bytes = element.to_bytes();
                        while let Some(byte) = bytes.pop() {
                            buffer.insert(0, byte);
                        }
                    }
                    let mut bytes = len.to_bytes();
                    while let Some(byte) = bytes.pop() {
                        buffer.insert(0, byte)
                    }
                    return None;
                }
            }
        }
        #[cfg(target_os = "linux")]
        use std::os::unix::ffi::OsStringExt;
        #[cfg(target_os = "windows")]
        use std::os::windows::ffi::OsStringExt;

        #[cfg(target_os = "linux")]
        return Some(OsString::from_vec(buff));
        #[cfg(target_os = "windows")]
        return Some(OsString::from_wide(&buff));
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use crate::TBytes;
    #[test]
    fn os_string() {
        let a = OsString::from("Hello World");
        let mut bytes = a.to_bytes();

        let b = OsString::from_bytes(&mut bytes).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn incomplite() {
        let mut buffer = Vec::new();
        buffer.append(&mut 4usize.to_bytes());
        buffer.push(b'g');
        buffer.push(0);
        buffer.push(b'r');
        buffer.push(0);
        buffer.push(b'a');
        buffer.push(0);
        let clone_buffer = buffer.clone();

        let other_buffer = OsString::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);

        buffer.push(b'y');
        buffer.push(0);
        let value = OsString::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, OsString::from("gray"))
    }
}
