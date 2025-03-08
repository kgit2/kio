#[macro_export]
macro_rules! type_wrapper {
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
        impl TypeWrapper {
            $(
                pub fn $e(self) -> $t {
                    match self {
                        TypeWrapper::$v(value) => value,
                        _ => panic!("TypeWrapper is not a {}", stringify!($v)),
                    }
                }
            )*
        }

        $(
            impl From<$t> for TypeWrapper {
                fn from(value: $t) -> Self {
                    TypeWrapper::$v(value)
                }
            }
        )*
    };
}
