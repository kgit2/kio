use crate::error::RioError;
use crate::ffi_util::string_into_c_char;
use crate::rio::rio_read::RioRead;
use crate::rio::rio_read_ref::RioReadRef;
use crate::rio::rio_write::RioWrite;
use crate::rio::rio_write_ref::RioWriteRef;
use crate::typed_result;

pub type Result<T> = std::result::Result<T, RioError>;

#[repr(C)]
pub enum RioResult {
    UnitVariant,
    Int8Variant(i8),
    UInt8Variant(u8),
    ShortVariant(i16),
    UShortVariant(u16),
    Int32Variant(i32),
    UInt32Variant(u32),
    Int64Variant(i64),
    UInt64Variant(u64),
    LongVariant(isize),
    ULongVariant(usize),
    FloatVariant(f32),
    DoubleVariant(f64),
    BooleanVariant(bool),
    ByteArrayVariant(*const i8),
    StringVariant(*const std::ffi::c_char),
    RioWriteRefVariant(RioWriteRef),
    RioReadRefVariant(RioReadRef),
    ErrorVariant(*const std::ffi::c_char),
}

typed_result! {
    Include {
        Int8Variant(i8, to_i8),
        UInt8Variant(u8, to_u8),
        ShortVariant(i16, to_short),
        UShortVariant(u16, to_ushort),
        Int32Variant(i32, to_int),
        UInt32Variant(u32, to_uint),
        Int64Variant(i64, to_i64),
        UInt64Variant(u64, to_u64),
        LongVariant(isize, to_long),
        ULongVariant(usize, to_ulong),
        FloatVariant(f32, to_float),
        DoubleVariant(f64, to_double),
        BooleanVariant(bool, to_boolean),
        ByteArrayVariant(*const i8, to_byte_array),
    }
    Exclude {
        // String(*const std::ffi::c_char, to_string),
        // COpaquePointer(*mut std::ffi::c_void, to_c_opaque_pointer),
        // Error(*const std::ffi::c_void, to_error),
    }
}

impl From<String> for RioResult {
    fn from(value: String) -> Self {
        RioResult::StringVariant(string_into_c_char(value))
    }
}

impl From<RioWriteRef> for RioResult {
    fn from(value: RioWriteRef) -> Self {
        RioResult::RioWriteRefVariant(value)
    }
}

impl From<RioWrite> for RioResult {
    fn from(value: RioWrite) -> Self {
        RioResult::RioWriteRefVariant(value.into())
    }
}

impl From<RioReadRef> for RioResult {
    fn from(value: RioReadRef) -> Self {
        RioResult::RioReadRefVariant(value)
    }
}

impl From<RioRead> for RioResult {
    fn from(value: RioRead) -> Self {
        RioResult::RioReadRefVariant(value.into())
    }
}

impl From<RioError> for RioResult {
    fn from(value: RioError) -> Self {
        RioResult::ErrorVariant(string_into_c_char(value.to_string()))
    }
}

impl<T> From<Result<T>> for RioResult
where
    T: Into<RioResult>,
{
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(value) => value.into(),
        }
    }
}

impl From<()> for RioResult {
    fn from(_: ()) -> Self {
        RioResult::UnitVariant
    }
}
