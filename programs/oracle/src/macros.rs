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

/// Adds numbers checking for overflow.
macro_rules! checked_add {
    ($left:expr, $right:expr $(,)?) => {
        match ($left).checked_add($right) {
            Some(value) => Ok(value),
            None => Err(::solana_program::program_error::ProgramError::ArithmeticOverflow),
        }
    };
}

/// Subtracts numbers checking for overflow.
macro_rules! checked_sub {
    ($left:expr, $right:expr $(,)?) => {
        match ($left).checked_sub($right) {
            Some(value) => Ok(value),
            None => Err(::solana_program::program_error::ProgramError::ArithmeticOverflow),
        }
    };
}

/// Multiplies numbers checking for overflow.
macro_rules! checked_mul {
    ($left:expr, $right:expr $(,)?) => {
        match ($left).checked_mul($right) {
            Some(value) => Ok(value),
            None => Err(::solana_program::program_error::ProgramError::ArithmeticOverflow),
        }
    };
}
