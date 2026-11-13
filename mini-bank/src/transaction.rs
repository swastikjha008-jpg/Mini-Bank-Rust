//! One entry in an account's history.
//!
//! **Concepts:** an enum whose variants carry different values, `impl` on an
//! enum, pattern matching with `match`, or-patterns and `matches!`.

use crate::money::Money;

/// What happened to an account. Each variant holds exactly the data it needs:
/// a deposit only needs an amount, but a transfer also needs the other account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transaction {
    Deposit(Money),
    Withdrawal(Money),
    TransferOut { to: u32, amount: Money },
    TransferIn { from: u32, amount: Money },
}

impl Transaction {
    /// The amount involved, whatever the kind. One arm handles all four
    /// variants because each binds a value named `amount` (an or-pattern).
    pub fn amount(&self) -> Money {
        match self {
            Transaction::Deposit(amount)
            | Transaction::Withdrawal(amount)
            | Transaction::TransferOut { amount, .. }
            | Transaction::TransferIn { amount, .. } => *amount,
        }
    }

    /// `true` if the money went *into* the account.
    pub fn is_credit(&self) -> bool {
        matches!(
            self,
            Transaction::Deposit(_) | Transaction::TransferIn { .. }
        )
    }

    /// The change to the balance in cents: positive for credits, negative
    /// for debits.
    pub fn signed_cents(&self) -> i128 {
        let cents = i128::from(self.amount().cents());
        if self.is_credit() {
            cents
        } else {
            -cents
        }
    }

    /// A one-line description for statements.
    pub fn describe(&self) -> String {
        match self {
            Transaction::Deposit(amount) => format!("deposit       +{amount}"),
            Transaction::Withdrawal(amount) => format!("withdrawal    -{amount}"),
            Transaction::TransferOut { to, amount } => {
                format!("transfer out  -{amount}  to #{to}")
            }
            Transaction::TransferIn { from, amount } => {
                format!("transfer in   +{amount}  from #{from}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cents(value: u64) -> Money {
        Money::from_cents(value)
    }

    #[test]
    fn amount_works_for_every_variant() {
        assert_eq!(Transaction::Deposit(cents(100)).amount(), cents(100));
        assert_eq!(Transaction::Withdrawal(cents(200)).amount(), cents(200));
        assert_eq!(
            Transaction::TransferOut {
                to: 1002,
                amount: cents(300)
            }
            .amount(),
            cents(300)
        );
        assert_eq!(
            Transaction::TransferIn {
                from: 1001,
                amount: cents(400)
            }
            .amount(),
            cents(400)
        );
    }

    #[test]
    fn credits_and_debits() {
        assert!(Transaction::Deposit(cents(1)).is_credit());
        assert!(Transaction::TransferIn {
            from: 1,
            amount: cents(1)
        }
        .is_credit());
        assert!(!Transaction::Withdrawal(cents(1)).is_credit());
        assert!(!Transaction::TransferOut {
            to: 1,
            amount: cents(1)
        }
        .is_credit());
    }

    #[test]
    fn signed_cents_has_the_right_sign() {
        assert_eq!(Transaction::Deposit(cents(250)).signed_cents(), 250);
        assert_eq!(Transaction::Withdrawal(cents(250)).signed_cents(), -250);
    }

    #[test]
    fn descriptions_mention_the_other_account() {
        let out = Transaction::TransferOut {
            to: 1002,
            amount: cents(2000),
        };
        assert_eq!(out.describe(), "transfer out  -20.00  to #1002");

        let incoming = Transaction::TransferIn {
            from: 1001,
            amount: cents(2000),
        };
        assert_eq!(incoming.describe(), "transfer in   +20.00  from #1001");
        assert_eq!(
            Transaction::Deposit(cents(5000)).describe(),
            "deposit       +50.00"
        );
    }
}
