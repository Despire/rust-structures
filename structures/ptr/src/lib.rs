#![feature(dispatch_from_dyn)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(negative_impls)]

pub use box_ptr::Box;
pub use rc_ptr::Rc;

pub mod box_ptr;
pub mod rc_ptr;
