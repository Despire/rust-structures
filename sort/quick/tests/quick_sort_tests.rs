use quick::sort;

#[test]
fn quick_sort_works() {
    let mut things = vec![1, 4, 4, 4, 3, 3];
    sort(&mut things);
    assert_eq!(things, vec![1, 3, 3, 4, 4, 4]);

    let mut things = vec![4, 1, 6, 3, 5, 2];
    sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4, 5, 6]);

    let mut things = vec![
        1, 5, 3, 2, 66, 7, 5234, 56, 4534, 65453, 45, 3543, 4325, 3432,
    ];
    sort(&mut things);
    assert_eq!(
        things,
        vec![1, 2, 3, 5, 7, 45, 56, 66, 3432, 3543, 4325, 4534, 5234, 65453]
    );
}
