pub mod error;
pub mod ffi_util;
pub mod result;
pub mod rio;
pub mod wrapper;
#[macro_use]
pub mod macros;
pub mod rfs;

#[no_mangle]
pub extern "C" fn init_backtrace() {
    color_eyre::install().expect("Failed to install color_eyre");
}

#[cfg(test)]
mod tests {
    use crate::init_backtrace;
    use color_eyre::eyre::{eyre, WrapErr};
    use color_eyre::Result;

    #[test]
    fn test_into_cstring() -> Result<()> {
        init_backtrace();
        println!(
            "{:#?}",
            Err::<(), _>(eyre!("This is an error message")).wrap_err("This is a wrap error")
        );
        Ok(())
    }
}
