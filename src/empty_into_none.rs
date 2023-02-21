use super::is_empty::IsEmpty;

/// Import this trait to add the `empty_into_none()` method to `Option<T>`,
/// where `T` implements [`IsEmpty`].
///
/// [`IsEmpty`] is implemented for the standard collections, and more.
///
/// [IsEmpty]: crate::is_empty::IsEmpty
pub trait EmptyIntoNone {
    /// If the value is `Option::Some` with an empty collection, returns
    /// `Option::None`. Otherwise, the original value is returned.
    ///
    /// # Examples
    ///
    /// These examples only show `Option<Vec<T>>`, but any `Option<T>` can be
    /// used where `T` implements [`IsEmpty`].
    ///
    /// `Some` with an empty `Vec` becomes `None`.
    /// ```
    /// # use optempty::EmptyIntoNone;
    /// #
    /// let some: Option<Vec<&str>> = Some(vec![]);
    /// let none = some.empty_into_none();
    /// assert_eq!(None, none);
    /// ```
    ///
    /// `Some` with a non-empty `Vec` remains unchanged.
    /// ```
    /// # use optempty::EmptyIntoNone;
    /// #
    /// let some = Some(vec!["a", "b", "c"]);
    /// let still_some = some.clone().empty_into_none();
    /// assert_eq!(some, still_some);
    /// ```
    ///
    /// `None` remains unchanged.
    /// ```
    /// # use optempty::EmptyIntoNone;
    /// #
    /// let none: Option<Vec<&str>> = None;
    /// let still_none = none.empty_into_none();
    /// assert_eq!(None, still_none);
    /// ```
    ///
    /// [IsEmpty]: crate::is_empty::IsEmpty
    fn empty_into_none(self) -> Self;
}

impl<T> EmptyIntoNone for Option<T>
where
    T: IsEmpty,
{
    fn empty_into_none(self) -> Self {
        self.and_then(|col| (!col.is_empty()).then_some(col))
    }
}
