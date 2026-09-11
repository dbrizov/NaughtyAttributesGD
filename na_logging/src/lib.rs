pub const PREFIX: &str = "[NaughtyAttributes]";

#[doc(hidden)]
pub mod __private {
    pub use godot::global::{godot_error, godot_print, godot_warn};
}

#[macro_export]
macro_rules! na_print {
    ($($arg:tt)+) => {
        $crate::__private::godot_print!("{} {}", $crate::PREFIX, ::std::format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! na_warn {
    ($($arg:tt)+) => {
        $crate::__private::godot_warn!("{} {}", $crate::PREFIX, ::std::format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! na_error {
    ($($arg:tt)+) => {
        $crate::__private::godot_error!("{} {}", $crate::PREFIX, ::std::format_args!($($arg)+))
    };
}
