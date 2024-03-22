#![allow(unused_macros)]

/// Print a message to the log.
macro_rules! log {
    ($($args:tt)*) => {
        match format_args!($($args)*) {
            args => match args.as_str() {
                Some(msg) => ::solana_program::log::sol_log(msg),
                None => ::solana_program::log::sol_log(&args.to_string()),
            }
        }
    };
}

/// Print an error message to the log.
macro_rules! err {
    ($($args:tt)*) => {
        ::solana_program::log::sol_log(&format!("Error: {}", format_args!($($args)*)))
    };
}
