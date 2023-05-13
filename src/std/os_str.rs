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
        let len = usize::from_bytes(buffer)?;
        let mut buff = Vec::with_capacity(len);
        for _ in 0..len {
            #[cfg(target_os = "linux")]
            {
                buff.push(buffer.next()?);
                buffer.next();
            }
            #[cfg(target_os = "windows")]
            buff.push(u16::from_bytes(buffer)?);
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

        let b = OsString::from_bytes(&mut bytes.drain(..)).unwrap();
        assert_eq!(a, b);
    }
}
