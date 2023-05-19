#![allow(clippy::question_mark)]
use crate::{TBuffer, TBytes};

macro_rules! impl_tuple {
    ($($T: ident | $t: ident | ($($t1: ident),*)),+; $($N: tt),+) => {
        impl<$($T: TBytes),*> TBytes for ($($T),*){
            fn size(&self) -> usize{
                $(self.$N.size() +)* 0
            }

            fn to_bytes(&self) -> Vec<u8>{
                let mut buffer = Vec::with_capacity(self.size());

                $(buffer.append(&mut self.$N.to_bytes());)*

                buffer
            }

            fn from_bytes(buffer: &mut TBuffer) -> Option<Self>{
                $(let $t = if let Some(value) = $T::from_bytes(buffer) {value}else{
                    $(
                        let mut bytes = $t1.to_bytes();
                        while let Some(byte) = bytes.pop(){
                            buffer.insert(0, byte);
                        }
                    )*
                    return None;
                };)*
                Some(($($t),*))
            }

        }
    };
}

impl_tuple!(T1 | t1 | (), T2 | t2 | (t1); 0, 1);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1); 0, 1, 2);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1); 0, 1, 2, 3);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1); 0, 1, 2, 3, 4);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1), T10 | t10 | (t9, t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1), T10 | t10 | (t9, t8, t7, t6, t5, t4, t3, t2, t1), T11 | t11 | (t10, t9, t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1), T10 | t10 | (t9, t8, t7, t6, t5, t4, t3, t2, t1), T11 | t11 | (t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T12 | t12 | (t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1), T10 | t10 | (t9, t8, t7, t6, t5, t4, t3, t2, t1), T11 | t11 | (t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T12 | t12 | (t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T13 | t13 | (t12, t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1), T10 | t10 | (t9, t8, t7, t6, t5, t4, t3, t2, t1), T11 | t11 | (t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T12 | t12 | (t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T13 | t13 | (t12, t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T14 | t14 | (t13, t12, t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_tuple!(T1 | t1 | (), T2 | t2 | (t1), T3 | t3 | (t2, t1), T4 | t4 | (t3, t2, t1), T5 | t5 | (t4, t3, t2, t1), T6 | t6 | (t5, t4, t3, t2, t1), T7 | t7 | (t6, t5, t4, t3, t2, t1), T8 | t8 | (t7, t6, t5, t4, t3, t2, t1), T9 | t9 | (t8, t7, t6, t5, t4, t3, t2, t1), T10 | t10 | (t9, t8, t7, t6, t5, t4, t3, t2, t1), T11 | t11 | (t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T12 | t12 | (t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T13 | t13 | (t12, t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T14 | t14 | (t13, t12, t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1), T15 | t15 | ( t14, t13, t12, t11, t10, t9, t8, t7, t6, t5, t4, t3, t2, t1); 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);

#[cfg(test)]
mod test {
    use crate::TBytes;

    #[test]
    fn i32_i64() {
        let a = (1i32, 53i64);

        let mut bytes = a.to_bytes();

        let b = <(i32, i64)>::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn result_i32_i64() {
        type R = Result<(i32, i64), ()>;

        let a: R = Ok((42i32, 21i64));

        let mut bytes = a.to_bytes();

        let b = R::from_bytes(&mut bytes).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn incomplete_t2() {
        let mut buffer = Vec::new();
        buffer.append(&mut String::from("Lomm").to_bytes());
        let clone_buffer = buffer.clone();

        let other_buffer = <(String, String)>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {other_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.append(&mut String::from("LOP").to_bytes());

        let value = <(String, String)>::from_bytes(&mut buffer).unwrap();
        assert_eq!(value, (String::from("Lomm"), String::from("LOP")))
    }
    #[test]
    fn incomplete_t15() {
        let mut buffer = Vec::new();
        let words = [
            "Lomm",
            "Doom",
            "Work",
            "I",
            "Hate",
            "Regular",
            "Expression",
            "I",
            "Want",
            "To",
            "Sleep",
            "I",
            "Don't",
            "Know",
            "What",
            "To",
            "Say",
        ];
        let strings = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();
        for string in &strings[0..14] {
            buffer.append(&mut string.to_bytes());
        }

        let clone_buffer = buffer.clone();

        let other_buffer = <(
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        )>::from_bytes(&mut buffer);
        if let Some(other_buffer) = other_buffer {
            panic!("This should be possible! Other buffer: {clone_buffer:?}");
        }

        assert_eq!(buffer, clone_buffer);
        buffer.append(&mut strings[14].to_bytes());

        let value = <(
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        )>::from_bytes(&mut buffer)
        .unwrap();

        if value.0 != strings[0] {
            panic!("Some thing went rong!")
        }
        if value.1 != strings[1] {
            panic!("Some thing went rong!")
        }
        if value.2 != strings[2] {
            panic!("Some thing went rong!")
        }
        if value.3 != strings[3] {
            panic!("Some thing went rong!")
        }
        if value.4 != strings[4] {
            panic!("Some thing went rong!")
        }
        if value.5 != strings[5] {
            panic!("Some thing went rong!")
        }
        if value.6 != strings[6] {
            panic!("Some thing went rong!")
        }
        if value.7 != strings[7] {
            panic!("Some thing went rong!")
        }
        if value.8 != strings[8] {
            panic!("Some thing went rong!")
        }
        if value.9 != strings[9] {
            panic!("Some thing went rong!")
        }
        if value.10 != strings[10] {
            panic!("Some thing went rong!")
        }
        if value.11 != strings[11] {
            panic!("Some thing went rong!")
        }
        if value.12 != strings[12] {
            panic!("Some thing went rong!")
        }
        if value.13 != strings[13] {
            panic!("Some thing went rong!")
        }
        if value.14 != strings[14] {
            panic!("Some thing went rong!")
        }
    }
}
