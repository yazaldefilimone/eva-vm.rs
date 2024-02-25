#[macro_export]
macro_rules! DIE {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }}
}
