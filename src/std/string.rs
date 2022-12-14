use crate::TBytes;

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

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let len = usize::from_bytes(buffer)?;
        let mut res = String::with_capacity(len);
        for _ in 0..len {
            res.push(buffer.pop()? as char);
        }
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
        b.reverse();

        let other = String::from_bytes(&mut b).unwrap();

        assert_eq!(a, other)
    }
}
