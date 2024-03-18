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

/// Try to unwrap a [`Result`] similar to `?`, but supporting `const fn`.
macro_rules! tri {
    ($opt:expr) => {
        match $opt {
            Some(value) => value,
            Err(err) => return Err(err),
        }
    };
}

/// Try to unwrap an [`Option`] similar to `?`, but supporting `const fn`.
macro_rules! tri_opt {
    ($opt:expr) => {
        match $opt {
            Some(value) => value,
            None => return None,
        }
    };
}

/// Increments a number by 1.
macro_rules! increment {
    ($value:expr, $amount:expr $(,)?) => {
        match ($value).checked_add($amount) {
            Some(value) => Ok(value),
            None => Err($crate::error::AuctionError::ArithmeticOverflow),
        }
    };
    ($value:expr) => {
        increment!($value, 1)
    };
}
