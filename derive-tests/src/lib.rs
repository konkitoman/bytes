use bytes_kman::prelude::*;

#[derive(Bytes, Debug, PartialEq)]
pub struct Test {
    a: i32,
    b: i64,
    c: String,
}

#[test]
fn incomplete_test() {
    let mut buffer = Vec::new();
    buffer.append(&mut 53.to_bytes());
    buffer.append(&mut 6666i64.to_bytes());
    let clone_buffer = buffer.clone();

    if let Some(_) = Test::from_bytes(&mut buffer) {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut String::from("Hello There!").to_bytes());

    let test = Test::from_bytes(&mut buffer).unwrap();

    assert_eq!(
        test,
        Test {
            a: 53,
            b: 6666,
            c: String::from("Hello There!")
        }
    )
}

#[derive(Bytes, Debug, PartialEq)]
pub struct Test2(String, i32, usize);

#[test]
fn incomplete_test2() {
    let mut buffer = Vec::new();
    buffer.append(&mut String::from("Hello There!").to_bytes());
    buffer.append(&mut 53.to_bytes());
    let clone_buffer = buffer.clone();

    if let Some(_) = Test2::from_bytes(&mut buffer) {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut 6666usize.to_bytes());

    let test = Test2::from_bytes(&mut buffer).unwrap();

    assert_eq!(test, Test2(String::from("Hello There!"), 53, 6666,))
}

#[derive(Bytes, Debug, PartialEq)]
pub enum Test3 {
    A(i32, String),
    B(String, usize, i32),
    C(usize),
    D { a: i32, b: String, q: usize },
    E { a: String },
    F,
}

#[test]
fn incomplete_test3() {
    // Test3::A
    let mut buffer = Vec::new();
    buffer.append(&mut 0usize.to_bytes());
    buffer.append(&mut 53.to_bytes());
    let clone_buffer = buffer.clone();

    if Test3::from_bytes(&mut buffer).is_some() {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut String::from("Hello There!").to_bytes());

    let test = Test3::from_bytes(&mut buffer).unwrap();

    assert_eq!(test, Test3::A(53, String::from("Hello There!")));

    // End Test3:A
    //
    // Test3::B

    let mut buffer = Vec::new();
    buffer.append(&mut 1usize.to_bytes());
    buffer.append(&mut String::from("Hello There!").to_bytes());
    buffer.append(&mut 53usize.to_bytes());
    let clone_buffer = buffer.clone();

    if Test3::from_bytes(&mut buffer).is_some() {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut 3i32.to_bytes());

    let test = Test3::from_bytes(&mut buffer).unwrap();

    assert_eq!(test, Test3::B(String::from("Hello There!"), 53, 3));
    // End Test3::B
    //
    // Test3::C

    let mut buffer = Vec::new();
    buffer.append(&mut 2usize.to_bytes());
    let clone_buffer = buffer.clone();

    if Test3::from_bytes(&mut buffer).is_some() {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut 53usize.to_bytes());

    let test = Test3::from_bytes(&mut buffer).unwrap();

    assert_eq!(test, Test3::C(53));

    // End Test3::C
    //
    // Test3::D

    let mut buffer = Vec::new();
    buffer.append(&mut 3usize.to_bytes());
    buffer.append(&mut 21.to_bytes());
    let clone_buffer = buffer.clone();

    if Test3::from_bytes(&mut buffer).is_some() {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut String::from("Hello").to_bytes());
    buffer.append(&mut 32usize.to_bytes());

    let test = Test3::from_bytes(&mut buffer).unwrap();

    assert_eq!(
        test,
        Test3::D {
            a: 21,
            b: String::from("Hello"),
            q: 32
        }
    );
    // End Test3::D
    //
    // Test3::E

    let mut buffer = Vec::new();
    buffer.append(&mut 4usize.to_bytes());
    let clone_buffer = buffer.clone();

    if Test3::from_bytes(&mut buffer).is_some() {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut String::from("Hello").to_bytes());

    let test = Test3::from_bytes(&mut buffer).unwrap();

    assert_eq!(
        test,
        Test3::E {
            a: String::from("Hello"),
        }
    );

    // End Test3::E
    //
    // Test3::F

    let mut buffer = Vec::new();
    let clone_buffer = Vec::new();

    if Test3::from_bytes(&mut buffer).is_some() {
        panic!("This should not happen because is incomplete!");
    }

    assert_eq!(buffer, clone_buffer);
    buffer.append(&mut 5usize.to_bytes());

    let test = Test3::from_bytes(&mut buffer).unwrap();

    assert_eq!(test, Test3::F)

    // End Test3::F
}
