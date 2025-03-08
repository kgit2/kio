use std::io::Read;

pub mod stderr;
pub mod stdin;
pub mod stdout;

pub fn mut_borrow_from_ptr<'a, T>(ptr: *mut std::ffi::c_void) -> &'a mut T {
    unsafe { &mut *(ptr as *mut T) }
}

pub fn read<R: Read>(reader: &mut R, buf: &mut [u8]) -> std::io::Result<usize> {
    reader.read(buf)
}
