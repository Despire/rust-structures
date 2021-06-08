use insertion::sort;

#[test]
fn insertion_sort_test() {
    let mut nmbrs = vec![4, 3, 2, 2, 5, 1];

    sort(&mut nmbrs);

    assert_eq!(nmbrs, vec![1, 2, 2, 3, 4, 5]);
}
