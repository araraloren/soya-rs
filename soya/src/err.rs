#[macro_export]
macro_rules! err {
        ($($arg:tt)*) => {
            $crate::aopt::raise_error!($($arg)*)
        };
    }

#[macro_export]
macro_rules! fail {
        ($($arg:tt)*) => {
            $crate::aopt::raise_failure!($($arg)*)
        };
    }
