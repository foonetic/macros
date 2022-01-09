use foonetic_macros;

mod some_sub_module {
    #[derive(Debug)]
    pub struct Thing {
        pub x: i8,
    }
}

#[derive(foonetic_macros::From, Debug)]
enum MyError {
    A(i8),
    B(String),
    C(some_sub_module::Thing),
}

fn my_fail() -> Result<i8, i8> {
    Err(1)
}

fn my_flaky_function(good: bool) -> Result<i8, MyError> {
    if good {
        return Ok(0);
    }
    Ok(my_fail()?)
}

#[test]
fn method_generation() {
    let something_i8 = MyError::from(2_i8);
    match something_i8 {
        MyError::A(x) => assert_eq!(x, 2),
        _ => panic!("bad match"),
    };

    let something_string = MyError::from(String::from("Hello, world"));
    match something_string {
        MyError::B(x) => assert_eq!("Hello, world", x),
        _ => panic!("bad match"),
    }
}

#[test]
fn question_operator() {
    assert_eq!(0, my_flaky_function(true).unwrap());
    match my_flaky_function(false) {
        Err(MyError::A(x)) => assert_eq!(x, 1),
        _ => panic!("bad match"),
    }
}
