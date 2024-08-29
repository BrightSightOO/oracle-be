#![allow(unused_macros)]

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
