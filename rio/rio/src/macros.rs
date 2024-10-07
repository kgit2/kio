#[macro_export]
macro_rules! typed_result {
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
        impl RioResult {
            $(
                #[no_mangle]
                pub extern "C" fn $e(self) -> $t {
                    match self {
                        RioResult::$v(value) => value,
                        _ => panic!("TypeWrapper is not a {}", stringify!($v)),
                    }
                }
            )*
        }

        $(
            impl From<$t> for RioResult {
                fn from(value: $t) -> Self {
                    RioResult::$v(value)
                }
            }
        )*
    };
}
