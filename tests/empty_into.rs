use std::collections::{BinaryHeap, LinkedList, VecDeque};

use maplit::*;
use optempty::{EmptyIntoErr, EmptyIntoNone};

fn check_option<T>(c: T)
where
    T: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
{
    let is_empty = c.is_empty();
    let col = Some(c);
    let into_none = col.clone().empty_into_none();
    if is_empty {
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

    assert!(Option::<T>::None.empty_into_none().is_none());
}

fn check_result<T>(col: T)
where
    T: optempty::is_empty::IsEmpty + std::fmt::Debug + Clone,
{
    fn empty_op() -> &'static str {
        "was empty"
    }

    let is_empty = col.is_empty();
    let col = Ok(col);
    let into_err = col.clone().empty_into_err(empty_op);
    if is_empty {
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

    let failed: Result<T, &str> = Err("failed");
    assert_eq!(
        failed.clone().unwrap_err(),
        failed.empty_into_err(empty_op).unwrap_err(),
        "Err should be unchanged (not treated as empty)",
    );
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
    let mut bh = BinaryHeap::new();
    check(bh.clone());

    bh.push("a");
    check(bh.clone());

    bh.push("b");
    bh.push("c");
    check(bh);
}

#[test]
fn btree_map() {
    let mut btm = btreemap! {};
    check(btm.clone());

    btm.insert("a", 1);
    check(btm.clone());

    btm.insert("b", 2);
    btm.insert("c", 3);
    check(btm.clone());
}

#[test]
fn btree_set() {
    let mut bts = btreeset! {};
    check(bts.clone());

    bts.insert("a");
    check(bts.clone());

    bts.insert("b");
    bts.insert("c");
    check(bts);
}

#[test]
fn hash_map() {
    let mut hm = hashmap! {};
    check(hm.clone());

    hm.insert("a", 1);
    check(hm.clone());

    hm.insert("b", 2);
    hm.insert("c", 3);
    check(hm.clone());
}

#[test]
fn hash_set() {
    let mut hs = hashset! {};
    check(hs.clone());

    hs.insert("a");
    check(hs.clone());

    hs.insert("b");
    hs.insert("c");
    check(hs);
}

#[test]
fn linked_list() {
    let mut ll = LinkedList::new();
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
    let mut vd = VecDeque::new();
    check(vd.clone());

    vd.push_back("a");
    check(vd.clone());

    vd.push_back("b");
    vd.push_back("c");
    check(vd);
}

#[test]
fn string() {
    check(String::new());
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

    let mut map = Map::new();
    check(map.clone());

    map.insert("a".into(), 1.into());
    check(map.clone());

    map.insert("b".into(), 2.into());
    map.insert("c".into(), 3.into());
    check(map);
}
