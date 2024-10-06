// #[repr(C)]
// pub enum TypeWrapper {
//     Int8(i8),
//     UInt8(u8),
//     Short(i16),
//     UShort(u16),
//     Int32(i32),
//     UInt32(u32),
//     Int64(i64),
//     UInt64(u64),
//     Long(isize),
//     ULong(usize),
//     Float(f32),
//     Double(f64),
//     Boolean(bool),
//     String(*const std::ffi::c_char),
//     COpaquePointer(*mut std::ffi::c_void),
//     Error(*const std::ffi::c_void),
// }

// type_wrapper! {
//     Include {
//         I8(i8, to_i8),
//         U8(u8, to_u8),
//         Short(i16, to_short),
//         UShort(u16, to_ushort),
//         Int(i32, to_int),
//         UInt(u32, to_uint),
//         I64(i64, to_i64),
//         U64(u64, to_u64),
//         Long(isize, to_long),
//         ULong(usize, to_ulong),
//         Float(f32, to_float),
//         Double(f64, to_double),
//         Boolean(bool, to_boolean),
//     }
//     Exclude {
//         String(*const std::ffi::c_char, to_string),
//         COpaquePointer(*mut std::ffi::c_void, to_c_opaque_pointer),
//         Error(*const std::ffi::c_void, to_error),
//     }
// }
