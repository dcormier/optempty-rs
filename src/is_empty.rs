use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// Used to determine if a collection, or `Option<T>` or `Result<T, E>` (where
/// `T` implements `IsEmpty`) is empty.
///
/// `IsEmpty` is implemented for the standard collections.
pub trait IsEmpty {
    /// Returns `true` if it is empty.
    fn is_empty(&self) -> bool;
}

/// Just wraps the existing `is_empty(&self)` method on the type.
// Because you can't spell `simple` without `impl`.
macro_rules! simple_is_empty {
    ($type:ident; $($args:tt)*) => {
        impl<$($args)*> $crate::is_empty::IsEmpty for $type<$($args)*> {
            fn is_empty(&self) -> bool {
                $type::is_empty(self)
            }
        }
    };
}

// Implement `IsEmpty` for the std collections.

simple_is_empty!(BinaryHeap; T);
simple_is_empty!(BTreeMap; K, V);
simple_is_empty!(BTreeSet; T);
simple_is_empty!(HashMap; K, V);
simple_is_empty!(HashSet; T);
simple_is_empty!(LinkedList; T);
simple_is_empty!(Vec; T);
simple_is_empty!(VecDeque; T);

impl<T> IsEmpty for Option<T>
where
    T: IsEmpty,
{
    /// Returns `true` if `Option` is `None` or `Some` with an empty `IsEmpty`,
    /// otherwise, `false`.
    ///
    /// # Examples
    ///
    /// Assumes you have `use optcollection::IsEmpty`.
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let some: Option<Vec<&str>> = Some(vec!["a", "b", "c"]);
    /// assert_eq!(false, some.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let some_empty: Option<Vec<&str>> = Some(vec![]);
    /// assert_eq!(true, some_empty.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let none: Option<Vec<&str>> = None;
    /// assert_eq!(true, none.is_empty());
    /// ```
    ///
    /// Works nested:
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let some_some: Option<Option<Vec<&str>>> = Some(Some(vec!["a", "b", "c"]));
    /// assert_eq!(false, some_some.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let some_some_empty: Option<Option<Vec<&str>>> = Some(Some(vec![]));
    /// assert_eq!(true, some_some_empty.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let some_none: Option<Option<Vec<&str>>> = Some(None);
    /// assert_eq!(true, some_none.is_empty());
    /// ```
    ///
    /// Works mixed:
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok_some_empty: Result<Option<Vec<&str>>, &str> = Ok(Some(vec![]));
    /// assert_eq!(true, ok_some_empty.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.as_ref().map(IsEmpty::is_empty).unwrap_or(true)
    }
}

impl<T, E> IsEmpty for Result<T, E>
where
    T: IsEmpty,
{
    /// Returns `true` if `Result` is `Err` or `Some` with an empty `IsEmpty`,
    /// otherwise, `false`.
    ///
    /// # Examples
    ///
    /// Assumes you have `use optcollection::IsEmpty`.
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok: Result<Vec<&str>, &str> = Ok(vec!["a", "b", "c"]);
    /// assert_eq!(false, ok.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok: Result<Vec<&str>, &str> = Ok(vec![]);
    /// assert_eq!(true, ok.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let err: Result<Vec<&str>, &str> = Err("nope");
    /// assert_eq!(true, err.is_empty());
    /// ```
    ///
    /// Works nested:
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok_ok: Result<Result<Vec<&str>, &str>, &str> = Ok(Ok(vec!["a", "b", "c"]));
    /// assert_eq!(false, ok_ok.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok_ok_empty: Result<Result<Vec<&str>, &str>, &str> = Ok(Ok(vec![]));
    /// assert_eq!(true, ok_ok_empty.is_empty());
    /// ```
    ///
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok_err: Result<Result<Vec<&str>, &str>, &str> = Ok(Err("none"));
    /// assert_eq!(true, ok_err.is_empty());
    /// ```
    ///
    /// Works mixed:
    /// ```
    /// # use optcollection::IsEmpty;
    /// #
    /// let ok_some_empty: Result<Option<Vec<&str>>, &str> = Ok(Some(vec![]));
    /// assert_eq!(true, ok_some_empty.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.as_ref().map(IsEmpty::is_empty).unwrap_or(true)
    }
}