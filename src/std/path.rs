use std::path::PathBuf;

use crate::{TBuffer, TBytes};

impl TBytes for PathBuf {
    fn size(&self) -> usize {
        self.as_os_str().to_os_string().size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.as_os_str().to_os_string().to_bytes()
    }

    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let str = std::ffi::OsString::from_bytes(buffer)?;
        Some(PathBuf::from(str))
    }
}
