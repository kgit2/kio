use crate::error::RioError;
use crate::result::Result;
use crate::result::RioResult;
use crate::rio::rio_read::RioRead;
use crate::rio::rio_write_ref::RioWriteRef;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Clone)]
pub struct RioReadRef {
    pub inner: *mut std::ffi::c_void,
}

impl From<RioRead> for RioReadRef {
    fn from(inner: RioRead) -> Self {
        RioReadRef {
            inner: inner.as_mut_ptr(),
        }
    }
}

impl Deref for RioReadRef {
    type Target = RioRead;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner as *mut RioRead) }
    }
}

impl DerefMut for RioReadRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.inner as *mut RioRead) }
    }
}

impl Drop for RioReadRef {
    fn drop(&mut self) {
        let value = RioRead::from_mut_ptr(self.inner);
        drop(value);
    }
}

impl RioReadRef {
    /// # Returns
    /// - [`RioResult::ErrorVariant`] if `byte_array` is null
    /// - [`RioResult::RioReadRefVariant`]::<[`RioRead::Stdin`]> otherwise
    #[no_mangle]
    pub extern "C" fn read_ref_new_byte_array(byte_array: *mut i8, size: usize) -> RioResult {
        if byte_array.is_null() {
            return RioError::NullPointerError("byte_array".to_string()).into();
        }
        let byte_array = unsafe { std::slice::from_raw_parts_mut(byte_array as *mut u8, size) };
        RioRead::new_byte_array(byte_array).into()
    }

    #[no_mangle]
    pub extern "C" fn read_ref_new_stdin() -> Self {
        RioRead::new_stdin().into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if `file` is null
    /// - [`RioResult::RioReadRefVariant`]::<[`RioRead::File`]> otherwise
    #[no_mangle]
    pub extern "C" fn read_ref_new_child_stdout(child_stdout: *mut std::ffi::c_void) -> RioResult {
        if child_stdout.is_null() {
            return RioError::NullPointerError("child_stdout".to_string()).into();
        }
        let child_stdout =
            unsafe { *Box::from_raw(child_stdout as *mut std::process::ChildStdout) };
        RioRead::new_child_stdout(child_stdout).into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if `file` is null
    /// - [`RioResult::RioReadRefVariant`]::<[`RioRead::File`]> otherwise
    #[no_mangle]
    pub extern "C" fn read_ref_new_child_stderr(child_stderr: *mut std::ffi::c_void) -> RioResult {
        if child_stderr.is_null() {
            return RioError::NullPointerError("child_stderr".to_string()).into();
        }
        let child_stderr =
            unsafe { *Box::from_raw(child_stderr as *mut std::process::ChildStderr) };
        RioRead::new_child_stderr(child_stderr).into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
    /// - [`RioResult::RioReadRefVariant`]::<[`RioRead::BufReader`]> otherwise
    #[no_mangle]
    pub extern "C" fn read_ref_new_buf_reader(mut self) -> RioResult {
        if self.inner.is_null() {
            return RioError::NullPointerError("buf_reader".to_string()).into();
        }
        let inner = unsafe { Box::from_raw(self.inner as *mut RioRead) };
        self.inner = std::ptr::null_mut();
        drop(self);
        RioRead::new_buf_reader(inner).into()
    }

    #[no_mangle]
    pub extern "C" fn read_ref_drop(self) {
        drop(self);
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if `buf` is null
    /// - [`RioResult::UnitVariant`] otherwise
    #[no_mangle]
    pub extern "C" fn read_ref_read(&mut self, buf: *mut i8, size: usize) -> RioResult {
        if buf.is_null() {
            return RioError::NullPointerError("buf".to_string()).into();
        }
        let result: Result<usize> = (|| {
            let buf = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };
            Ok(self.deref_mut().read(buf)?)
        })();
        result.into()
    }

    /// # Returns
    /// - [`RioResult::ByteArrayVariant`]::<[`std::ptr::null`]> if `size` is null
    /// - [`RioResult::ByteArrayVariant`] if `read_to_string` was successful
    /// - [`RioResult::ErrorVariant`] otherwise
    #[no_mangle]
    pub extern "C" fn read_ref_read_to_string(&mut self, size: &mut usize) -> RioResult {
        #[allow(useless_ptr_null_checks)]
        if (size as *mut usize).is_null() {
            return RioResult::ByteArrayVariant(std::ptr::null());
        }
        let result: Result<*const i8> = (|| {
            let mut buf = String::new();
            *size = self.deref_mut().read_to_string(&mut buf)?;
            let bytes: Vec<u8> = buf.into_bytes();
            Ok(bytes.as_ptr() as *const i8)
        })();
        result.into()
    }
}
