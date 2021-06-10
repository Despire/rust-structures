use std::thread;

trait A {}

struct B;

impl A for B {}

impl B {
    fn hello(&self) {
        println!("hello from B arc_test.rs");
    }
}

impl Drop for B {
    fn drop(&mut self) {
        println!("dropping B arc_test.rs");
    }
}

#[test]
fn arc_test() {
    let top_level = ptr::Arc::new(B);
    let mut threads = Vec::with_capacity(10);

    for _ in 0..10 {
        let inner_level = ptr::Arc::clone(&top_level);

        threads.push(thread::spawn(move || {
            inner_level.hello();
        }))
    }

    for t in threads {
        t.join();
    }
}
