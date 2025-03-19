pub trait IntoFFI {
    type FFIType;
    fn into_ffi(self) -> Self::FFIType;
}

pub trait FromFFI {
    type OriginRef: ?Sized; // 引用类型（允许 unsized）
    type OriginOwned; // 所有权类型

    // 零拷贝转换
    fn as_origin(&self) -> &Self::OriginRef;

    // 所有权转移转换
    fn into_origin(self) -> Self::OriginOwned;
}
