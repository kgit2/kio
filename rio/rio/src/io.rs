pub mod ffi_read;
pub mod ffi_write;
pub mod stderr;
pub mod stdin;
pub mod stdout;

pub fn mut_borrow_from_ptr<'a, T>(ptr: *mut std::ffi::c_void) -> &'a mut T {
    unsafe { &mut *(ptr as *mut T) }
}
