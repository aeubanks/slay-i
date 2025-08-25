#[macro_export]
macro_rules! assert_matches {
    ($e1:expr, $e2:pat) => {
        assert!(matches!($e1, $e2))
    };
}
