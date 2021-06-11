#[test]
fn into_iter_test() {
    let mut v = vec::Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut iter = v.into_iter();
    assert_eq!(iter.next().unwrap(), 1);
    assert_eq!(iter.next_back().unwrap(), 3);
    assert_eq!(iter.next().unwrap(), 2);
    assert_eq!(iter.next_back(), None);
}
