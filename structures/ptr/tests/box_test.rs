trait A {}

struct B;
impl A for B {}

impl B {
    fn hello(&self) {
        println!("hello");
    }
}

struct T(i32);

#[test]
fn test_box() {
    let mut b = ptr::Box::new(T(5));
    b.0 = 10;

    let b = ptr::Box::new(B);

    b.hello();

    dynamic(b);

}

fn dynamic(b: ptr::Box<dyn A>) {}
