#![forbid(unsafe_code)]

#[macro_export]
macro_rules! deque {
    () => (
        ::std::collections::VecDeque::new()
    );
    ($elem:expr; $n:expr) => (
        ::std::collections::VecDeque::from(::std::vec![$elem; $n])
    );
    ($($x:expr),+ $(,)?) => (
        ::std::collections::VecDeque::from(::std::vec![$($x),+])
    );
}

#[macro_export]
macro_rules! sorted_vec {
    () => {
        ::std::vec![]
    };
    ($elem:expr; $n:expr) => {
        ::std::vec![$elem; $n]
    };
    ($($elem:expr),+ $(,)?) => {
        {
            let mut vec = ::std::vec![$($elem),+];
        vec.sort();
        vec
        }
    };
}
#[macro_export]
macro_rules! map {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $val:expr),* $(,)?) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    };
}
