#![allow(unused_macros)]

macro_rules! account_schema_tests {
    ($($account:ident)*) => {
        #[cfg(test)]
        mod account_schema_tests {
            $(
                #[allow(non_snake_case)]
                mod $account {
                    use crate::state::$account;

                    #[test]
                    fn validate() {
                        $crate::utils::tests::validate_schema::<$account>();
                    }

                    #[test]
                    fn max_size() {
                        $crate::utils::tests::validate_max_size::<$account>();
                    }
                }
            )*
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
