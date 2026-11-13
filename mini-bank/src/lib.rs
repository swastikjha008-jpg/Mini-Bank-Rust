//! # mini-bank
//!
//! A tiny bank simulator built to practise core Rust ideas:
//!
//! - **Borrowing and references:** [`bank`] and [`account`]
//! - **Structs and `impl` blocks:** [`money`], [`account`], [`bank`]
//! - **Enums, pattern matching, enums with values, `impl` on enums:**
//!   [`transaction`], [`command`], [`error`]
//! - **`Result` and `Option`:** everywhere, see [`error`] and [`bank`]
//!
//! ```
//! use mini_bank::bank::Bank;
//! use mini_bank::money::Money;
//!
//! let mut bank = Bank::new();
//! let alice = bank.open("Alice").unwrap();
//! let bob = bank.open("Bob").unwrap();
//!
//! bank.deposit(alice, "100".parse().unwrap()).unwrap();
//! bank.transfer(alice, bob, "25.50".parse().unwrap()).unwrap();
//!
//! assert_eq!(bank.account(bob).unwrap().balance(), Money::from_cents(2550));
//! assert!(bank.account(9999).is_err());
//! ```

pub mod account;
pub mod app;
pub mod bank;
pub mod command;
pub mod error;
pub mod money;
pub mod transaction;

pub use error::{BankError, Result};
