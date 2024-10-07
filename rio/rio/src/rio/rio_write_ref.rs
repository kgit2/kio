use crate::error::RioError;
use crate::result::Result;
use crate::result::RioResult;
use crate::rio::rio_write::RioWrite;
use std::io::Write;
use std::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Clone)]
pub struct RioWriteRef {
    pub inner: *mut std::ffi::c_void,
}

impl From<RioWrite> for RioWriteRef {
    fn from(inner: RioWrite) -> Self {
        RioWriteRef {
            inner: inner.as_mut_ptr(),
        }
    }
}

impl Deref for RioWriteRef {
    type Target = RioWrite;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner as *mut RioWrite) }
    }
}

impl DerefMut for RioWriteRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.inner as *mut RioWrite) }
    }
}

impl Drop for RioWriteRef {
    fn drop(&mut self) {
        if self.inner.is_null() {
            return;
        }
        drop(RioWrite::from_mut_ptr(self.inner));
    }
}

impl RioWriteRef {
    #[no_mangle]
    pub extern "C" fn write_ref_new_stdout() -> Self {
        RioWrite::new_stdout().into()
    }

    #[no_mangle]
    pub extern "C" fn write_ref_new_stderr() -> Self {
        RioWrite::new_stderr().into()
    }

    /// # Safety
    /// The caller must ensure that the byte_array is valid for the duration of the TypedWriteRef.
    /// # Returns
    /// - [`RioResult::ErrorVariant`] if `byte_array` is null
    /// - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::ByteArray`]> otherwise
    #[no_mangle]
    pub unsafe extern "C" fn write_ref_new_byte_array(
        byte_array: *mut i8,
        len: usize,
    ) -> RioResult {
        if byte_array.is_null() {
            return RioError::NullPointerError("byte_array".to_string()).into();
        }
        let byte_array = std::slice::from_raw_parts_mut(byte_array as *mut u8, len);
        RioWrite::new_byte_array(byte_array).into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if `child_stdin` is null
    /// - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::File`]> otherwise
    #[no_mangle]
    pub extern "C" fn write_ref_new_child_stdin(child_stdin: *mut std::ffi::c_void) -> RioResult {
        if child_stdin.is_null() {
            return RioError::NullPointerError("child_stdin".to_string()).into();
        }
        let child_stdin = unsafe { *Box::from_raw(child_stdin as *mut std::process::ChildStdin) };
        RioWrite::new_child_stdin(child_stdin).into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
    /// - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::BufWriter`]> otherwise
    #[no_mangle]
    pub extern "C" fn write_ref_new_buf_writer(mut self) -> RioResult {
        if self.inner.is_null() {
            return RioError::NullPointerError("inner".to_string()).into();
        }
        let inner = unsafe { Box::from_raw(self.inner as *mut RioWrite) };
        self.inner = std::ptr::null_mut();
        drop(self);
        RioWrite::new_buf_writer(inner).into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
    /// - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::LineWriter`]> otherwise
    #[no_mangle]
    pub extern "C" fn write_ref_new_line_writer(mut self) -> RioResult {
        if self.inner.is_null() {
            return RioError::NullPointerError("inner".to_string()).into();
        }
        let inner = unsafe { Box::from_raw(self.inner as *mut RioWrite) };
        self.inner = std::ptr::null_mut();
        drop(self);
        RioResult::RioWriteRefVariant(RioWrite::new_line_writer(inner).into())
    }

    #[no_mangle]
    pub extern "C" fn write_ref_drop(self) {
        drop(self);
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
    /// - [`RioResult::ErrorVariant`] if `buf` is null
    /// - [`RioResult::UnitVariant`] if write was successful.
    #[no_mangle]
    pub extern "C" fn write_ref_write(&mut self, buf: *const i8, size: usize) -> RioResult {
        if self.inner.is_null() {
            return RioError::NullPointerError("inner".to_string()).into();
        }
        if buf.is_null() {
            return RioError::NullPointerError("buf".to_string()).into();
        }

        let result: Result<usize> = (|| {
            let slice = unsafe { std::slice::from_raw_parts(buf as *const u8, size) };
            Ok(self.deref_mut().write(slice)?)
        })();
        result.into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
    /// - [`RioResult::UnitVariant`] if flush was successful.
    #[no_mangle]
    pub extern "C" fn write_ref_flush(&mut self) -> RioResult {
        if self.inner.is_null() {
            return RioError::NullPointerError("inner".to_string()).into();
        }
        let result: Result<()> = (|| Ok(self.deref_mut().flush()?))();
        result.into()
    }

    /// # Returns
    /// - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
    /// - [`RioResult::ErrorVariant`] if `buf` is null
    /// - [`RioResult::UnitVariant`] if `write_all` was successful.
    #[no_mangle]
    pub extern "C" fn write_ref_write_all(
        &mut self,
        buf: *const std::ffi::c_char,
        size: usize,
    ) -> RioResult {
        if self.inner.is_null() {
            return RioError::NullPointerError("inner".to_string()).into();
        }
        if buf.is_null() {
            return RioError::NullPointerError("buf".to_string()).into();
        }

        let result: Result<()> = (|| {
            let slice = unsafe { std::slice::from_raw_parts(buf as *const u8, size) };
            self.deref_mut().write_all(slice)?;
            Ok(())
        })();
        result.into()
    }
}
