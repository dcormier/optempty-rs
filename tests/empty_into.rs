extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

use optempty::{EmptyIntoErr, EmptyIntoNone};

fn check_option<T>(col: T)
where
    T: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
{
    fn internal_check<U>(col: Option<U>, inner_is_empty: bool)
    where
        U: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
    {
        let col = Some(col);
        let into_none = col.clone().empty_into_none();
        if inner_is_empty {
            assert!(
                into_none.is_none(),
                "Should be None: {:?}.empty_into_none()\n\
                 Got:            {:?}",
                col,
                into_none,
            );
        } else {
            assert!(
                into_none.is_some(),
                "Should be Some: {:?}.empty_into_none()\n\
                 Got:            {:?}",
                col,
                into_none,
            );
        }

        assert!(Option::<U>::None.empty_into_none().is_none());
    }

    let inner_is_empty = col.is_empty();
    // Check borrowed values
    internal_check(Some(&col), inner_is_empty);
    // Check owned values
    internal_check(Some(col), inner_is_empty);
}

fn check_result<T>(col: T)
where
    T: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
{
    fn internal_check<U, E>(col: Result<U, E>, inner_is_empty: bool)
    where
        U: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
        E: std::fmt::Debug + Clone,
    {
        fn empty_op() -> &'static str {
            "was empty"
        }

        let col = Ok(col);
        let into_err = col.clone().empty_into_err(empty_op);
        if inner_is_empty {
            assert_eq!(
                Result::<&str, _>::Err("was empty"),
                Result::<&str, _>::Err(into_err.clone().unwrap_err()),
                "Should be Err(\"was empty\"): {:?}.empty_into_err(|| \"was empty\")\n\
             Got:                          {:?}",
                col,
                into_err,
            );
        } else {
            assert!(
                into_err.is_ok(),
                "Should be Ok: {:?}.empty_into_err(|| \"was empty\")\n\
             Got:          {:?}",
                col,
                into_err,
            );
        }

        let failed: Result<U, &str> = Err("failed");
        assert_eq!(
            failed.clone().unwrap_err(),
            failed.empty_into_err(empty_op).unwrap_err(),
            "Err should be unchanged (not treated as empty)",
        );
    }

    let inner_is_empty = col.is_empty();
    // Check borrowed values
    internal_check::<_, &str>(Ok(&col), inner_is_empty);
    // Check owned values
    internal_check::<_, &str>(Ok(col), inner_is_empty);
}

fn check<T>(col: T)
where
    T: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
{
    check_option(col.clone());
    check_result(col);
}

#[test]
fn binary_heap() {
    let mut bh = BinaryHeap::default();
    check(bh.clone());

    bh.push("a");
    check(bh.clone());

    bh.push("b");
    bh.push("c");
    check(bh);
}

#[test]
fn btree_map() {
    let mut btm = BTreeMap::default();
    check(btm.clone());

    btm.insert("a", 1);
    check(btm.clone());

    btm.insert("b", 2);
    btm.insert("c", 3);
    check(btm.clone());
}

#[test]
fn btree_set() {
    let mut bts = BTreeSet::default();
    check(bts.clone());

    bts.insert("a");
    check(bts.clone());

    bts.insert("b");
    bts.insert("c");
    check(bts);
}

#[cfg(feature = "std")]
#[test]
fn hash_map() {
    let mut hm = std::collections::HashMap::default();
    check(hm.clone());

    hm.insert("a", 1);
    check(hm.clone());

    hm.insert("b", 2);
    hm.insert("c", 3);
    check(hm.clone());
}

#[cfg(feature = "std")]
#[test]
fn hash_set() {
    let mut hs = std::collections::HashSet::default();
    check(hs.clone());

    hs.insert("a");
    check(hs.clone());

    hs.insert("b");
    hs.insert("c");
    check(hs);
}

#[test]
fn linked_list() {
    let mut ll = LinkedList::default();
    check(ll.clone());

    ll.push_back("a");
    check(ll.clone());

    ll.push_back("b");
    ll.push_back("c");
    check(ll);
}

#[test]
fn vec() {
    let mut v = vec![];
    check(v.clone());

    v.push("a");
    check(v.clone());

    v.push("b");
    v.push("c");
    check(v);
}

#[test]
fn vec_deque() {
    let mut vd = VecDeque::default();
    check(vd.clone());

    vd.push_back("a");
    check(vd.clone());

    vd.push_back("b");
    vd.push_back("c");
    check(vd);
}

#[test]
fn string() {
    check(String::default());
    check(String::from("a"));
    check(String::from("abc"));
}

#[test]
fn str() {
    check("");
    check("a");
    check("abc");
}

#[test]
fn slice() {
    check("".as_bytes());
    check("a".as_bytes());
    check("abc".as_bytes());
}

#[cfg(feature = "serdejson")]
#[test]
fn serde_json_map() {
    use serde_json::Map;

    let mut map = Map::default();
    check(map.clone());

    map.insert("a".into(), 1.into());
    check(map.clone());

    map.insert("b".into(), 2.into());
    map.insert("c".into(), 3.into());
    check(map);
}
