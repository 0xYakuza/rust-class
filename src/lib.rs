#[macro_export]
macro_rules! class {
    ($(class $name:ident {[$($param_type:ident $param:ident)*]; $($fn_return:ident $fname:ident ($($arg:tt)*) {$($block:tt)*})*})*) => {
        $(struct $name {
            $(
                pub $param: $param_type,
            )*
        }
        impl $name {
            $(pub fn $fname ($($arg)*) -> $fn_return {
                $($block)*
            })*
        })*
    }
}