use std::io::BufReader;

pub mod stderr;
pub mod stdin;
pub mod stdout;

pub fn mut_borrow_from_ptr<'a, T>(ptr: *mut std::ffi::c_void) -> &'a mut T {
    BufReader
    unsafe { &mut *(ptr as *mut T) }
}
