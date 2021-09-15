/// A trait to add a helper method to `Option<Vec<T>>` to convert `Some(vec![])`
/// (empty `Vec`) into `None`.
///
/// Once you `use optvec::EmptyIntoNone`, you gain the `empty_into_none()`
/// method on `Option<Vec<T>>`.
///
/// # Examples
///
/// `Some` with an empty `Vec` becomes `None`.
/// ```
/// # use optvec::EmptyIntoNone;
/// #
/// let some: Option<Vec<&str>> = Some(vec![]);
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `Some` with a non-empty `Vec` remains unchanged.
/// ```
/// # use optvec::EmptyIntoNone;
/// #
/// let some = Some(vec!["a", "b", "c"]);
/// let still_some = some.clone().empty_into_none();
/// assert_eq!(some, still_some);
/// ```
///
/// `None` remains unchanged.
/// ```
/// # use optvec::EmptyIntoNone;
/// #
/// let none: Option<Vec<&str>> = None;
/// let still_none = none.empty_into_none();
/// assert_eq!(None, still_none);
/// ```
pub trait EmptyIntoNone {
    /// If the value is `Some(vec![])` (an empty `Vec`), returns `None`.
    /// Otherwise the original value is returned.
    fn empty_into_none(self) -> Self;
}

impl<T> EmptyIntoNone for Option<Vec<T>> {
    fn empty_into_none(self) -> Self {
        self.and_then(|v| (!v.is_empty()).then(|| v))
    }
}
