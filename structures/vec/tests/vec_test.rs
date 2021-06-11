struct B(i32);

impl Drop for B {
    fn drop(&mut self) {
        println!("dropping B in vec_test.rs");
    }
}

#[test]
fn vec_with_drop_objects() {
    let mut v = vec::Vec::new();

    v.push(B(5));
    v.push(B(5));
    v.push(B(5));
}

#[test]
fn vec_push() {
    let mut v: vec::Vec<i32> = vec::Vec::new();

    v.push(1);
    assert_eq!(v.capacity(), 1);
    v.push(2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.capacity(), 4);
}

#[test]
fn vec_pop() {
    let mut v: vec::Vec<i32> = vec::Vec::new();

    v.push(1);
    assert_eq!(v.capacity(), 1);
    v.push(2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.capacity(), 4);

    v.pop();
    assert_eq!(v.len(), 2);
    v.pop();
    assert_eq!(v.len(), 1);
    assert_eq!(v.capacity(), 4);
}

#[test]
fn vec_insert() {
    let mut v: vec::Vec<i32> = vec::Vec::new();

    v.insert(0, 5);
    v.insert(0, 10);
    v.insert(1, 15);

    assert_eq!(v[0], 10);
    assert_eq!(v[1], 15);
    assert_eq!(v[2], 5);
}

#[test]
fn vec_remove() {
    let mut v: vec::Vec<i32> = vec::Vec::new();

    v.insert(0, 5);
    v.insert(0, 10);
    v.insert(1, 15);

    assert_eq!(v[0], 10);
    assert_eq!(v[1], 15);
    assert_eq!(v[2], 5);

    v.remove(1);
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 5);
    assert_eq!(v.len(), 2);

    v.remove(1);
    assert_eq!(v[0], 10);
    assert_eq!(v.len(), 1);
}
