extern crate alloc;

use optempty::IsEmpty;

#[test]
fn binary_heap() {
    let mut bh = alloc::collections::BinaryHeap::default();
    assert!(IsEmpty::is_empty(&bh));

    bh.push("a");
    assert!(!IsEmpty::is_empty(&bh));
}

#[test]
fn btree_map() {
    let mut btm = alloc::collections::BTreeMap::default();
    assert!(IsEmpty::is_empty(&btm));

    btm.insert("a", "b");
    assert!(!IsEmpty::is_empty(&btm));
}

#[test]
fn btree_set() {
    let mut bts = alloc::collections::BTreeSet::default();
    assert!(IsEmpty::is_empty(&bts));

    bts.insert("a");
    assert!(!IsEmpty::is_empty(&bts));
}

#[cfg(feature = "std")]
#[test]
fn hash_map() {
    let mut hm = std::collections::HashMap::default();
    assert!(IsEmpty::is_empty(&hm));

    hm.insert("a", "b");
    assert!(!IsEmpty::is_empty(&hm));
}

#[cfg(feature = "std")]
#[test]
fn hash_set() {
    let mut hs = std::collections::HashSet::default();
    assert!(IsEmpty::is_empty(&hs));

    hs.insert("a");
    assert!(!IsEmpty::is_empty(&hs));
}

#[test]
fn linked_list() {
    let mut ll = alloc::collections::LinkedList::default();
    assert!(IsEmpty::is_empty(&ll));

    ll.push_back("a");
    assert!(!IsEmpty::is_empty(&ll));
}

#[test]
fn vec() {
    let mut v = vec![];
    assert!(IsEmpty::is_empty(&v));

    v.push("a");
    assert!(!IsEmpty::is_empty(&v));
}

#[test]
fn vec_deque() {
    let mut vd = alloc::collections::VecDeque::default();
    assert!(IsEmpty::is_empty(&vd));

    vd.push_back("a");
    assert!(!IsEmpty::is_empty(&vd));
}

#[test]
fn string() {
    assert!(IsEmpty::is_empty(&String::default()));
    assert!(!IsEmpty::is_empty(&String::from("a")));
}

#[test]
fn str() {
    assert!(IsEmpty::is_empty(&""));
    assert!(!IsEmpty::is_empty(&"a"));
}

#[test]
fn slice() {
    assert!(IsEmpty::is_empty(&"".as_bytes()));
    assert!(!IsEmpty::is_empty(&"a".as_bytes()));
}

#[cfg(feature = "serdejson")]
#[test]
fn serde_json_map() {
    use serde_json::Map;

    let mut map = Map::default();
    assert!(IsEmpty::is_empty(&map));

    map.insert("a".into(), 1.into());
    assert!(!IsEmpty::is_empty(&map));
}
