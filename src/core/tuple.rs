use crate::TBytes;

macro_rules! impl_tuple {
    ($($T: ident),+; $($N: tt),+) => {
        impl<$($T: TBytes),*> TBytes for ($($T),*){
            fn size(&self) -> usize{
                $(self.$N.size() +)* 0
            }

            fn to_bytes(&self) -> Vec<u8>{
                let mut buffer = Vec::with_capacity(self.size());

                $(buffer.append(&mut self.$N.to_bytes());)*

                buffer
            }

            fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>{
                Some(($($T::from_bytes(buffer)?),*))
            }

        }
    };
}

impl_tuple!(T1, T2; 0, 1);
impl_tuple!(T1, T2, T3; 0, 1, 2);
impl_tuple!(T1, T2, T3, T4; 0, 1, 2, 3);
impl_tuple!(T1, T2, T3, T4, T5; 0, 1, 2, 3, 4);
impl_tuple!(T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8; 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn i32_i64() {
        let a = (1i32, 53i64);

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let b = <(i32, i64)>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn result_i32_i64() {
        type R = Result<(i32, i64), ()>;

        let a: R = Ok((42i32, 21i64));

        let mut bytes = a.to_bytes();
        bytes.reverse();

        let b = R::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b);
    }
}
