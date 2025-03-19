pub mod ffi_bytes;
pub mod ffi_string;
pub mod ffi_vec;

use crate::ffi_handle::FFIHandle;
use crate::ffi_value;
use crate::ffi_value::ffi_bytes::FFIBytes;
use crate::ffi_value::ffi_string::FFIString;
use crate::ffi_value::ffi_vec::FFIVec;

pub trait IntoFFIValue {
    fn into_ffi_value(self) -> FFIValue;
}

#[repr(C)]
#[derive(Debug)]
pub enum FFIValue {
    Int8(i8),
    UInt8(u8),
    Short(i16),
    UShort(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Long(isize),
    ULong(usize),
    Float(f32),
    Double(f64),
    Boolean(bool),
    String(FFIString),
    Bytes(FFIBytes),
    Handle(FFIHandle),
    Vec(FFIVec),
    // Result(*mut FFIResult),
    // Option(*mut FFIOption),
    Unit,
}

ffi_value! {
    Include {
        Int8(i8, to_i8),
        UInt8(u8, to_u8),
        Short(i16, to_short),
        UShort(u16, to_ushort),
        Int32(i32, to_int),
        UInt32(u32, to_uint),
        Int64(i64, to_i64),
        UInt64(u64, to_u64),
        Long(isize, to_long),
        ULong(usize, to_ulong),
        Float(f32, to_float),
        Double(f64, to_double),
        Boolean(bool, to_boolean),
    }
    Exclude {
        String(FFIString, to_string),
        Bytes(FFIBytes, to_bytes),
        Vec(FFIVec, to_vec),
        Unit((), to_unit),
    }
}

// impl Drop for FFIValue {
//     fn drop(&mut self) {
//         match self {
//             FFIValue::Result(result) => {
//                 if !result.is_null() {
//                     drop(unsafe { Box::from_raw(result) });
//                 }
//             }
//             FFIValue::Option(option) => {
//                 if !option.is_null() {
//                     drop(unsafe { Box::from_raw(option) });
//                 }
//             }
//             _ => {}
//         }
//     }
// }
