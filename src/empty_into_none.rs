use super::is_empty::IsEmpty;

/// A trait to add the `empty_into_none()` method to `Option<T>` (where `T`
/// implements [`IsEmpty`]) to convert `Option::Some` with an empty [`IsEmpty`]
/// into `Option::None`.
///
/// `IsEmpty` is implemented for the standard collections.
///
/// # Examples
///
/// `Option::Some` with an empty `Vec` becomes `Option::None`.
/// ```
/// use optempty::EmptyIntoNone;
///
/// let some: Option<Vec<&str>> = Some(vec![]);
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `Option::Some` with a non-empty `Vec` remains unchanged.
/// ```
/// use optempty::EmptyIntoNone;
///
/// let some = Some(vec!["a", "b", "c"]);
/// let still_some = some.clone().empty_into_none();
/// assert_eq!(some, still_some);
/// ```
///
/// `None` remains unchanged.
/// ```
/// use optempty::EmptyIntoNone;
///
/// let none: Option<Vec<&str>> = None;
/// let still_none = none.empty_into_none();
/// assert_eq!(None, still_none);
/// ```
///
/// Works out of the box with all types from [`std::collections`]:
///
/// `BinaryHeap`
/// ```
/// use std::collections::BinaryHeap;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<BinaryHeap<&str>> = Some(BinaryHeap::new());
/// let none = some.empty_into_none();
/// assert!(none.is_none());
/// ```
///
/// `BTreeMap`
/// ```
/// use std::collections::BTreeMap;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<BTreeMap<&str, &str>> = Some(BTreeMap::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `BTreeSet`
/// ```
/// use std::collections::BTreeSet;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<BTreeSet<&str>> = Some(BTreeSet::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `HashMap`
/// ```
/// use std::collections::HashMap;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<HashMap<&str, &str>> = Some(HashMap::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `HashSet`
/// ```
/// use std::collections::HashSet;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<HashSet<&str>> = Some(HashSet::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `LinkedList`
/// ```
/// use std::collections::LinkedList;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<LinkedList<&str>> = Some(LinkedList::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `VecDeque`
/// ```
/// use std::collections::VecDeque;
///
/// use optempty::EmptyIntoNone;
///
/// let some: Option<VecDeque<&str>> = Some(VecDeque::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// ## Other Types
/// Also works for other types that can be empty.
///
/// `String`:
/// ```
/// use optempty::EmptyIntoNone;
///
/// let some: Option<String> = Some(String::new());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `&str`:
/// ```
/// use optempty::EmptyIntoNone;
///
/// let some: Option<&str> = Some("");
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// `&[T]` (or `slice`):
/// ```
/// use optempty::EmptyIntoNone;
///
/// let some: Option<&[u8]> = Some("".as_bytes());
/// let none = some.empty_into_none();
/// assert_eq!(None, none);
/// ```
///
/// [IsEmpty]: crate::is_empty::IsEmpty
/// [std::collections]: https://doc.rust-lang.org/std/collections/
pub trait EmptyIntoNone {
    /// If the value is `Option::Some` with an empty collection, returns
    /// `Option::None`. Otherwise the original value is returned.
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
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };

    use maplit::*;

    use super::EmptyIntoNone;

    fn check<C>(col: Option<C>, should_be_some: bool)
    where
        C: crate::is_empty::IsEmpty + std::fmt::Debug + Clone,
    {
        let into_none = col.clone().empty_into_none();
        if should_be_some {
            assert!(
                into_none.is_some(),
                "Should be Some: {:?}.empty_into_none()\n\
                 Got:            {:?}",
                col,
                into_none,
            );
        } else {
            assert!(
                into_none.is_none(),
                "Should be None: {:?}.empty_into_none()\n\
                 Got:            {:?}",
                col,
                into_none,
            );
        }
    }

    #[test]
    fn binary_heap() {
        let mut bh = BinaryHeap::<&str>::new();
        bh.push("a");
        check(Some(bh.clone()), true);

        bh.push("b");
        bh.push("c");
        check(Some(bh), true);
        check(Some(BinaryHeap::<&str>::new()), false);
        check(Option::<BinaryHeap<&str>>::None, false);
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

    #[test]
    fn string() {
        check(Some(String::from("a")), true);
        check(Some(String::new()), false);
        check(Option::<String>::None, false);
    }

    #[test]
    fn str() {
        check(Some("a"), true);
        check(Some(""), false);
        check(Option::<String>::None, false);
    }

    #[test]
    fn slice() {
        check(Some("a".as_bytes()), true);
        check(Some("".as_bytes()), false);
        check(Option::<&[u8]>::None, false);
    }
}
