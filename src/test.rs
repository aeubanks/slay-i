#[macro_export]
macro_rules! assert_matches {
    ($e1:expr, $e2:pat) => {
        assert!(matches!($e1, $e2))
    };
}

#[macro_export]
macro_rules! assert_not_matches {
    ($e1:expr, $e2:pat) => {
        assert!(!matches!($e1, $e2))
    };
}

#[cfg(test)]
mod test_impl {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    fn iter_to_count_map<T: Hash + Eq, I>(i: I) -> HashMap<T, i32>
    where
        I: IntoIterator<Item = T>,
    {
        i.into_iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(v).or_default() += 1;
            m
        })
    }

    pub fn assert_set_eq<T: Hash + Eq + Debug, A, B>(a: A, b: B)
    where
        A: IntoIterator<Item = T>,
        B: IntoIterator<Item = T>,
    {
        assert_eq!(iter_to_count_map(a), iter_to_count_map(b));
    }
}

#[cfg(test)]
pub use test_impl::assert_set_eq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_set_eq_pos() {
        assert_set_eq([] as [u32; _], []);
        assert_set_eq([1, 2, 3], [1, 2, 3]);
        assert_set_eq([1, 2, 3], [2, 3, 1]);
        assert_set_eq([1, 2, 3, 3], [2, 3, 1, 3]);
    }

    #[test]
    #[should_panic]
    fn test_assert_set_eq_neg() {
        assert_set_eq(&[1, 2, 3], &[1, 2]);
    }
}
