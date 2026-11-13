//! Everything that can go wrong, as one enum.
//!
//! **Concepts:** enums with values (`InsufficientFunds { .. }`, `AccountNotFound(u32)`),
//! implementing a trait (`Display`) for an enum, and a `Result` type alias.

use std::fmt;

use crate::money::Money;

/// A bank error. Some variants carry data so the message can be specific.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BankError {
    EmptyName,
    EmptyCommand,
    InvalidAmount(String),
    InvalidId(String),
    ZeroAmount,
    InsufficientFunds { needed: Money, available: Money },
    Overflow,
    AccountNotFound(u32),
    SameAccount,
    UnknownCommand(String),
    Usage(&'static str),
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `match` must cover every variant, so adding a new error later
        // makes the compiler point at every place that needs updating.
        match self {
            BankError::EmptyName => write!(f, "the account holder's name cannot be empty"),
            BankError::EmptyCommand => write!(f, "type a command (or `help`)"),
            BankError::InvalidAmount(text) => write!(
                f,
                "'{text}' is not a valid amount (use a number like 25 or 25.50)"
            ),
            BankError::InvalidId(text) => write!(f, "'{text}' is not a valid account id"),
            BankError::ZeroAmount => write!(f, "the amount must be greater than zero"),
            BankError::InsufficientFunds { needed, available } => write!(
                f,
                "insufficient funds: need {needed} but only {available} available"
            ),
            BankError::Overflow => write!(f, "that would exceed the maximum balance"),
            BankError::AccountNotFound(id) => write!(f, "no account with id {id}"),
            BankError::SameAccount => write!(f, "cannot transfer to the same account"),
            BankError::UnknownCommand(word) => {
                write!(f, "unknown command '{word}' (type `help`)")
            }
            BankError::Usage(usage) => write!(f, "usage: {usage}"),
        }
    }
}

impl std::error::Error for BankError {}

/// Shorthand used across the crate: `Result<T>` means `Result<T, BankError>`.
pub type Result<T> = std::result::Result<T, BankError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_are_helpful() {
        let err = BankError::InsufficientFunds {
            needed: Money::from_cents(5000),
            available: Money::from_cents(1250),
        };
        assert_eq!(
            err.to_string(),
            "insufficient funds: need 50.00 but only 12.50 available"
        );
        assert_eq!(
            BankError::AccountNotFound(1007).to_string(),
            "no account with id 1007"
        );
        assert_eq!(
            BankError::Usage("deposit <id> <amount>").to_string(),
            "usage: deposit <id> <amount>"
        );
    }

    #[test]
    fn works_as_a_std_error() {
        let boxed: Box<dyn std::error::Error> = Box::new(BankError::ZeroAmount);
        assert_eq!(boxed.to_string(), "the amount must be greater than zero");
    }
}
