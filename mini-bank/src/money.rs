//! Money, stored as whole cents.
//!
//! **Concepts:** a tuple struct, an `impl` block with an associated constant,
//! `Option` returned by checked arithmetic, and `FromStr` for parsing text.
//!
//! Computers cannot store 0.10 exactly as a float, so money is kept as an
//! integer number of cents instead (12.50 is stored as 1250).

use std::fmt;
use std::str::FromStr;

use crate::error::BankError;

/// An amount of money in cents. It can never be negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Money(u64);

impl Money {
    /// Zero cents. An *associated constant*: `Money::ZERO`.
    pub const ZERO: Money = Money(0);

    pub const fn from_cents(cents: u64) -> Self {
        Money(cents)
    }

    pub const fn cents(self) -> u64 {
        self.0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// `Some(sum)`, or `None` if the sum would not fit.
    pub fn checked_add(self, other: Money) -> Option<Money> {
        self.0.checked_add(other.0).map(Money)
    }

    /// `Some(difference)`, or `None` if `other` is larger (no negative money).
    pub fn checked_sub(self, other: Money) -> Option<Money> {
        self.0.checked_sub(other.0).map(Money)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `pad` makes width flags such as `{:>10}` work on this type.
        f.pad(&format!("{}.{:02}", self.0 / 100, self.0 % 100))
    }
}

impl FromStr for Money {
    type Err = BankError;

    /// Accepts `25`, `25.5`, `25.50` and `.5`. Rejects signs, letters,
    /// more than two decimals and amounts too large to store.
    ///
    /// ```
    /// use mini_bank::money::Money;
    ///
    /// let price: Money = "12.50".parse().unwrap();
    /// assert_eq!(price.cents(), 1250);
    /// assert!("12.345".parse::<Money>().is_err());
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let text = input.trim();
        let invalid = || BankError::InvalidAmount(text.to_string());

        let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
        let all_digits = |part: &str| part.chars().all(|c| c.is_ascii_digit());

        let nothing_there = whole.is_empty() && fraction.is_empty();
        if nothing_there || fraction.len() > 2 || !all_digits(whole) || !all_digits(fraction) {
            return Err(invalid());
        }

        let units: u64 = if whole.is_empty() {
            0
        } else {
            whole.parse().map_err(|_| invalid())?
        };
        let cents: u64 = match fraction.len() {
            0 => 0,
            1 => fraction.parse::<u64>().map_err(|_| invalid())? * 10,
            _ => fraction.parse().map_err(|_| invalid())?,
        };

        units
            .checked_mul(100)
            .and_then(|total| total.checked_add(cents))
            .map(Money)
            .ok_or_else(invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Result<Money, BankError> {
        text.parse()
    }

    #[test]
    fn parses_common_formats() {
        assert_eq!(parse("12.50").unwrap().cents(), 1250);
        assert_eq!(parse("12").unwrap().cents(), 1200);
        assert_eq!(parse("0.5").unwrap().cents(), 50);
        assert_eq!(parse(".5").unwrap().cents(), 50);
        assert_eq!(parse("0.05").unwrap().cents(), 5);
        assert_eq!(parse("  7.25 ").unwrap().cents(), 725);
        assert_eq!(parse("0").unwrap(), Money::ZERO);
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            "",
            ".",
            "abc",
            "-5",
            "+5",
            "1.234",
            "1,000",
            "1.2.3",
            "12 50",
            "999999999999999999999",
            "184467440737095517",
        ] {
            assert!(
                matches!(parse(bad), Err(BankError::InvalidAmount(_))),
                "{bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn largest_storable_amount_still_parses() {
        assert!(parse("184467440737095516.15").is_ok());
    }

    #[test]
    fn display_always_shows_two_decimals() {
        assert_eq!(Money::from_cents(5).to_string(), "0.05");
        assert_eq!(Money::from_cents(1250).to_string(), "12.50");
        assert_eq!(Money::from_cents(123_456).to_string(), "1234.56");
        assert_eq!(Money::ZERO.to_string(), "0.00");
        assert_eq!(format!("[{:>8}]", Money::from_cents(1250)), "[   12.50]");
    }

    #[test]
    fn checked_math_returns_options() {
        let ten = Money::from_cents(1000);
        let three = Money::from_cents(300);
        assert_eq!(ten.checked_add(three), Some(Money::from_cents(1300)));
        assert_eq!(ten.checked_sub(three), Some(Money::from_cents(700)));
        assert_eq!(three.checked_sub(ten), None);
        assert_eq!(Money::from_cents(u64::MAX).checked_add(three), None);
    }

    #[test]
    fn money_is_ordered() {
        assert!(Money::from_cents(1) < Money::from_cents(2));
        assert_eq!(Money::default(), Money::ZERO);
        assert!(Money::ZERO.is_zero());
    }
}
