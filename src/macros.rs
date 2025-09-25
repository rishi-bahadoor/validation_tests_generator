#[macro_export]
macro_rules! print_warn_ln {
    ($($arg:tt)*) => {
        color_print::cprintln!("<yellow>[WARN]</> {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_help_ln {
    ($($arg:tt)*) => {
        color_print::cprintln!("<blue>[HELPER]</> {}", format!($($arg)*));
    };
}
