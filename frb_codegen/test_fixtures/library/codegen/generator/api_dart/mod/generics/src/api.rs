// Test comprehensive generic support implementation

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

/// Generic struct with constraints (currently simplified)
pub struct Wrapper<T> 
where 
    T: Clone,
{
    pub data: T,
    pub is_valid: bool,
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

/// Functions using generic types
pub fn process_container(container: Container<String>) -> i32 {
    container.count
}

pub fn create_pair(first: i32, second: String) -> Pair<i32, String> {
    Pair { first, second }
}

pub fn handle_option(opt: Option<bool>) -> bool {
    match opt {
        Option::Some(value) => value,
        Option::None => false,
    }
}

pub fn swap_either(either: Either<String, i32>) -> Either<i32, String> {
    match either {
        Either::Left(s) => Either::Right(s.len() as i32),
        Either::Right(i) => Either::Left(i.to_string()),
    }
}
