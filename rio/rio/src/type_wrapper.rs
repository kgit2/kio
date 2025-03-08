use crate::byte_array::ArrayBuffer;
use crate::type_wrapper;

#[repr(C)]
pub enum TypeWrapper {
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
    String(*mut std::ffi::c_char),
    Array(ArrayBuffer),
    COpaquePointer(*mut std::ffi::c_void),
    Unit,
}

type_wrapper! {
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
        String(*mut std::ffi::c_char, to_string),
        Array(ArrayBuffer, to_array_buffer),
        COpaquePointer(*mut std::ffi::c_void, to_c_opaque_pointer),
        Unit((), to_unit),
    }
}

impl From<*mut std::ffi::c_char> for TypeWrapper {
    fn from(value: *mut std::ffi::c_char) -> Self {
        TypeWrapper::String(value)
    }
}

impl From<ArrayBuffer> for TypeWrapper {
    fn from(value: ArrayBuffer) -> Self {
        TypeWrapper::Array(value)
    }
}
