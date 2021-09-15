use crate::is_empty::IsEmpty;

/// A trait to add the `empty_into_none()` method to `Option<T>` (where `T`
/// implements [IsEmpty]) to convert `Some` with an empty [IsEmpty] into `None`.
///
/// `IsEmpty` is implemented for the standard collections.
///
/// # Examples
///
/// `Some` with an empty `Vec` becomes `None`.
/// ```
/// # use optcollection::EmptyIntoNone;
/// #
/// let some: Option<Vec<&str>> = Some(vec![]);
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `Some` with a non-empty `Vec` remains unchanged.
/// ```
/// # use optcollection::EmptyIntoNone;
/// #
/// let some = Some(vec!["a", "b", "c"]);
/// let still_some = some.clone().empty_into_none();
/// assert_eq!(some, still_some);
/// ```
///
/// `None` remains unchanged.
/// ```
/// # use optcollection::EmptyIntoNone;
/// #
/// let none: Option<Vec<&str>> = None;
/// let still_none = none.empty_into_none();
/// assert_eq!(None, still_none);
/// ```
///
/// [IsEmpty]: crate::is_empty::IsEmpty
pub trait EmptyIntoNone {
    /// If the value is `Some` with an empty collection, returns `None`.
    /// Otherwise the original value is returned.
    fn empty_into_none(self) -> Self;
}

impl<T> EmptyIntoNone for Option<T>
where
    T: IsEmpty,
{
    fn empty_into_none(self) -> Self {
        self.and_then(|col| (!col.is_empty()).then(|| col))
    }
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

    use maplit::*;

    use super::EmptyIntoNone;

    fn check<C>(col: Option<C>, should_be_some: bool)
    where
        C: crate::is_empty::IsEmpty + std::fmt::Debug + Clone + Eq,
    {
        if should_be_some {
            let expect = col.clone();
            assert_eq!(expect, col.empty_into_none());
        } else {
            assert_eq!(None, col.empty_into_none());
        }
    }

    #[test]
    fn btree_map() {
        check(Some(btreemap! {"a" => 1}), true);
        check(Some(btreemap! {"a" => 1, "b" => 2, "c" => 3}), true);
        check(Some(BTreeMap::<&str, i8>::new()), false);
        check(Option::<BTreeMap<&str, i8>>::None, false);
    }

    #[test]
    fn btree_set() {
        check(Some(btreeset! {"a"}), true);
        check(Some(btreeset! {"a", "b", "c"}), true);
        check(Some(BTreeSet::<&str>::new()), false);
        check(Option::<BTreeSet<&str>>::None, false);
    }

    #[test]
    fn hash_map() {
        check(Some(hashmap! {"a" => 1}), true);
        check(Some(hashmap! {"a" => 1, "b" => 2, "c" => 3}), true);
        check(Some(HashMap::<&str, i8>::new()), false);
        check(Option::<HashMap<&str, i8>>::None, false);
    }

    #[test]
    fn hash_set() {
        check(Some(hashset! {"a"}), true);
        check(Some(hashset! {"a", "b", "c"}), true);
        check(Some(HashSet::<&str>::new()), false);
        check(Option::<HashSet<&str>>::None, false);
    }

    #[test]
    fn linked_list() {
        let mut ll = LinkedList::new();
        ll.push_back("a");
        check(Some(ll.clone()), true);

        ll.push_back("b");
        ll.push_back("c");
        check(Some(ll), true);
        check(Some(LinkedList::<&str>::new()), false);
        check(Option::<LinkedList<&str>>::None, false);
    }

    #[test]
    fn vec() {
        check(Some(vec!["a"]), true);
        check(Some(vec!["a", "b", "c"]), true);
        check(Some(Vec::<&str>::new()), false);
        check(Option::<Vec<&str>>::None, false);
    }

    #[test]
    fn vec_deque() {
        check(Some(VecDeque::from(vec!["a"])), true);
        check(Some(VecDeque::from(vec!["a", "b", "c"])), true);
        check(Some(VecDeque::<&str>::new()), false);
        check(Option::<VecDeque<&str>>::None, false);
    }
}
