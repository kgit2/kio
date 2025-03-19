// use crate::ffi_value::{FFIValue, IntoFFIValue};
//
// #[repr(C)]
// #[derive(Debug)]
// pub enum FFIOption {
//     Some(FFIValue),
//     None,
// }
//
// impl FFIOption {
//     pub fn some(value: FFIValue) -> Self {
//         Self::Some(value)
//     }
//
//     pub fn none() -> Self {
//         Self::None
//     }
//
//     #[no_mangle]
//     pub extern "C" fn is_some(&self) -> bool {
//         matches!(self, Self::Some(_))
//     }
//
//     #[no_mangle]
//     pub extern "C" fn is_none(&self) -> bool {
//         matches!(self, Self::None)
//     }
//
//     #[no_mangle]
//     pub extern "C" fn option_unwrap(self) -> FFIValue {
//         match self {
//             Self::Some(value) => value,
//             Self::None => panic!("called `FFIOption::unwrap()` on a `None` value"),
//         }
//     }
// }
//
