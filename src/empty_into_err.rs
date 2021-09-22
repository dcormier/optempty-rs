use super::is_empty::IsEmpty;

/// Import this trait to add the `empty_into_err(op)` method to `Result<T, E>`,
/// where `T` implements [`IsEmpty`].
///
/// [`IsEmpty`] is implemented for the standard collections, and more.
///
/// [IsEmpty]: crate::is_empty::IsEmpty
pub trait EmptyIntoErr<T, E> {
    /// If the value is `Result::Ok` with an inner value that [`IsEmpty::is_empty`],
    /// calls `op` and returns its return value in `Result::Err`. Otherwise the
    /// original value is returned.
    ///
    /// # Examples
    ///
    /// These examples only show `Result<Vec<T>, E>`, but any `Result<T, E>` can
    /// be used where `T` implements [`IsEmpty`].
    ///
    /// `Ok` with an empty `Vec` becomes `Err`.
    /// ```
    /// # use optempty::EmptyIntoErr;
    /// #
    /// let ok: Result<Vec<&str>, &str> = Ok(vec![]);
    /// let err = ok.empty_into_err(|| "was empty");
    /// assert_eq!(Err("was empty"), err);
    /// ```
    ///
    /// `Ok` with a non-empty `Vec` remains unchanged.
    /// ```
    /// # use optempty::EmptyIntoErr;
    /// #
    /// let ok = Ok(vec!["a", "b", "c"]);
    /// let still_ok = ok.empty_into_err(|| "was empty");
    /// assert_eq!(Ok(vec!["a", "b", "c"]), still_ok);
    /// ```
    ///
    /// `Err` remains unchanged.
    /// ```
    /// # use optempty::EmptyIntoErr;
    /// #
    /// let err: Result<Vec<&str>, &str> = Err("failed");
    /// let still_err = err.empty_into_err(|| "was empty");
    /// assert_eq!(Err("failed"), still_err);
    /// ```
    ///
    /// [IsEmpty]: crate::is_empty::IsEmpty
    fn empty_into_err<O>(self, op: O) -> Result<T, E>
    where
        O: FnOnce() -> E;
}

impl<T, E> EmptyIntoErr<T, E> for Result<T, E>
where
    T: IsEmpty,
{
    fn empty_into_err<O>(self, op: O) -> Result<T, E>
    where
        O: FnOnce() -> E,
    {
        if self.is_empty() {
            Err(op())
        } else {
            self
        }
    }
}
