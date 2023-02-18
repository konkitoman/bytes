// use std::os::unix::prelude::{OsStrExt, OsStringExt};
use std::ffi::OsString;

use crate::TBytes;

impl TBytes for std::ffi::OsString {
    fn size(&self) -> usize {
        (self.len() * if cfg!(target_os = "windows") { 2 } else { 1 }) + 0usize.size()
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
            buffer.push(*byte)
        }
        #[cfg(target_os = "windows")]
        for byte in self.encode_wide() {
            buffer.append(&mut byte.to_bytes())
        }

        buffer
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let len = usize::from_bytes(buffer)?;
        let mut buff = Vec::with_capacity(len);
        for _ in 0..len {
            #[cfg(target_os = "linux")]
            buff.push(buffer.pop()?);
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

        None
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
        bytes.reverse();

        let b = OsString::from_bytes(&mut bytes).unwrap();
        assert_eq!(a, b);
    }
}
