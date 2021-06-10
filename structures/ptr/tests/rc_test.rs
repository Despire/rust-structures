trait A {}

struct B;
impl A for B {}

impl Drop for B {
    fn drop(&mut self) {
        println!("dropping B");
    }
}

#[test]
fn test_rc_ptr() {
    let first = ptr::Rc::new(B);

    assert_eq!(1, ptr::Rc::strong_count(&first));

    let second = ptr::Rc::clone(&first);

    assert_eq!(2, ptr::Rc::strong_count(&first));

    consume_rc(second);

    assert_eq!(1, ptr::Rc::strong_count(&first));
}

fn consume_rc(rc: ptr::Rc<dyn A>) {}
