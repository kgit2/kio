#[macro_export]
macro_rules! ffi_value {
    (
        Include {
            $(
                $v:ident($t:ty, $e:ident),
            )*
        }
        Exclude {
            $(
                $ev:ident($et:ty, $ee:ident),
            )*
        }
    ) => {
        impl FFIValue {
            $(
                pub fn $e(self) -> $t {
                    match self {
                        FFIValue::$v(value) => value,
                        _ => panic!("FFIValue is not a {}", stringify!($v)),
                    }
                }
            )*
        }

        $(
            impl From<$t> for FFIValue {
                fn from(value: $t) -> Self {
                    FFIValue::$v(value)
                }
            }
        )*

        $(
            impl IntoFFIValue for $t {
                fn into_ffi_value(self) -> FFIValue {
                    FFIValue::$v(self)
                }
            }
        )*
    };
}
