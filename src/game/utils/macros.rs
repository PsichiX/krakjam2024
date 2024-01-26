#[macro_export]
macro_rules! hash_set {
    ($($value:expr),*) => {
        {
            let mut result = std::collections::HashSet::new();
            $(
                result.insert($value);
            )*
            result
        }
    };
}

#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut result = std::collections::HashMap::new();
            $(
                result.insert($key, $value);
            )*
            result
        }
    };
}
