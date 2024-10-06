// use crate::type_wrapper::TypeWrapper;
//
// #[repr(C)]
// pub enum TypedResult {
//     Ok(TypeWrapper),
//     Err(TypeWrapper),
// }
//
// impl TypedResult {
//     #[no_mangle]
//     pub extern "C" fn is_oK(&self) -> bool {
//         matches!(self, TypedResult::Ok(_))
//     }
//
//     #[no_mangle]
//     pub extern "C" fn is_err(&self) -> bool {
//         matches!(self, TypedResult::Err(_))
//     }
//
//     #[no_mangle]
//     pub extern "C" fn unwrap(self) -> TypeWrapper {
//         match self {
//             TypedResult::Ok(value) => value,
//             TypedResult::Err(error) => error,
//         }
//     }
//
//     #[no_mangle]
//     pub extern "C" fn unwrap_err(self) -> TypeWrapper {
//         match self {
//             TypedResult::Ok(value) => value,
//             TypedResult::Err(error) => error,
//         }
//     }
// }
