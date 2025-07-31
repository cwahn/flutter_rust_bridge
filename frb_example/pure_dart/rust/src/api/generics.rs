// Generic types testing for Flutter Rust Bridge

/// Basic generic struct with single type parameter
pub struct Container<T> {
    pub value: T,
    pub count: i32,
}

/// Generic struct with multiple type parameters
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

/// Generic enum with single type parameter
pub enum Option<T> {
    Some(T),
    None,
}

/// Generic enum with multiple type parameters
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// Functions to test generic types

pub fn create_string_container() -> Container<String> {
    Container {
        value: "hello".to_string(),
        count: 1,
    }
}

pub fn create_int_container() -> Container<i32> {
    Container {
        value: 42,
        count: 1,
    }
}

pub fn process_string_container(container: Container<String>) -> i32 {
    container.count + container.value.len() as i32
}

pub fn create_pair() -> Pair<i32, String> {
    Pair { 
        first: 10, 
        second: "world".to_string() 
    }
}

pub fn handle_string_option(opt: Option<String>) -> String {
    match opt {
        Option::Some(value) => value,
        Option::None => "empty".to_string(),
    }
}

pub fn handle_int_either(either: Either<i32, String>) -> String {
    match either {
        Either::Left(num) => format!("number: {}", num),
        Either::Right(text) => format!("text: {}", text),
    }
}

// Test nested generics
pub fn create_nested_container() -> Container<Option<String>> {
    Container {
        value: Option::Some("nested".to_string()),
        count: 1,
    }
}

pub fn process_nested_container(container: Container<Option<String>>) -> String {
    match container.value {
        Option::Some(s) => s,
        Option::None => "none".to_string(),
    }
}
