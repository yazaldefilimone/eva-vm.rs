#[macro_export]
macro_rules! DIE {
    ($($arg:tt)*) => {{
        eprintln!("FATAL ERROR: {}", format!($($arg)*));
        std::process::exit(1);
    }};
}
