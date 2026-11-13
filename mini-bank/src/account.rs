//! A single bank account.
//!
//! **Concepts:** a struct with private fields, `impl` blocks, `&self` (shared
//! borrow) versus `&mut self` (exclusive borrow), getters that return
//! references, `Result` for things that can fail, and `Option` for things that
//! might not exist.

use crate::error::{BankError, Result};
use crate::money::Money;
use crate::transaction::Transaction;

/// An account owns its name, its balance and its history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    id: u32,
    owner: String,
    balance: Money,
    history: Vec<Transaction>,
}

impl Account {
    /// Create an empty account.
    pub fn new(id: u32, owner: impl Into<String>) -> Self {
        Account {
            id,
            owner: owner.into(),
            balance: Money::ZERO,
            history: Vec::new(),
        }
    }

    // ---- Reading: `&self` lends the account out read-only. -----------------

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns a *reference* into the account instead of copying the text.
    /// The reference cannot outlive the account.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn balance(&self) -> Money {
        self.balance
    }

    /// A borrowed view of the whole history. Nothing is copied.
    pub fn history(&self) -> &[Transaction] {
        &self.history
    }

    /// `None` when nothing has happened yet.
    pub fn last_transaction(&self) -> Option<&Transaction> {
        self.history.last()
    }

    /// The biggest single deposit, or `None` if there are no deposits.
    pub fn largest_deposit(&self) -> Option<Money> {
        self.history
            .iter()
            .filter_map(|transaction| match transaction {
                Transaction::Deposit(amount) => Some(*amount),
                _ => None,
            })
            .max()
    }

    /// Would `amount` fit in this account without overflowing?
    pub fn can_receive(&self, amount: Money) -> bool {
        self.balance.checked_add(amount).is_some()
    }

    // ---- Changing: `&mut self` borrows the account exclusively. ------------

    pub fn deposit(&mut self, amount: Money) -> Result<()> {
        self.credit(amount)?;
        self.history.push(Transaction::Deposit(amount));
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Money) -> Result<()> {
        self.debit(amount)?;
        self.history.push(Transaction::Withdrawal(amount));
        Ok(())
    }

    /// Money leaving for another account. Used by `Bank::transfer`.
    pub(crate) fn send(&mut self, to: u32, amount: Money) -> Result<()> {
        self.debit(amount)?;
        self.history.push(Transaction::TransferOut { to, amount });
        Ok(())
    }

    /// Money arriving from another account. Used by `Bank::transfer`.
    pub(crate) fn receive(&mut self, from: u32, amount: Money) -> Result<()> {
        self.credit(amount)?;
        self.history.push(Transaction::TransferIn { from, amount });
        Ok(())
    }

    // ---- Private helpers: the single place where the balance changes. ------

    fn credit(&mut self, amount: Money) -> Result<()> {
        if amount.is_zero() {
            return Err(BankError::ZeroAmount);
        }
        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        Ok(())
    }

    fn debit(&mut self, amount: Money) -> Result<()> {
        if amount.is_zero() {
            return Err(BankError::ZeroAmount);
        }
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(BankError::InsufficientFunds {
                needed: amount,
                available: self.balance,
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cents(value: u64) -> Money {
        Money::from_cents(value)
    }

    fn account() -> Account {
        Account::new(1001, "Asha")
    }

    #[test]
    fn new_account_is_empty() {
        let acct = account();
        assert_eq!(acct.id(), 1001);
        assert_eq!(acct.owner(), "Asha");
        assert_eq!(acct.balance(), Money::ZERO);
        assert!(acct.history().is_empty());
        assert_eq!(acct.last_transaction(), None);
        assert_eq!(acct.largest_deposit(), None);
    }

    #[test]
    fn deposit_and_withdraw_change_the_balance() {
        let mut acct = account();
        acct.deposit(cents(10_000)).unwrap();
        acct.withdraw(cents(2_550)).unwrap();
        assert_eq!(acct.balance(), cents(7_450));
        assert_eq!(acct.history().len(), 2);
        assert_eq!(
            acct.last_transaction(),
            Some(&Transaction::Withdrawal(cents(2_550)))
        );
    }

    #[test]
    fn cannot_overdraw() {
        let mut acct = account();
        acct.deposit(cents(1_000)).unwrap();

        let err = acct.withdraw(cents(1_001)).unwrap_err();
        assert_eq!(
            err,
            BankError::InsufficientFunds {
                needed: cents(1_001),
                available: cents(1_000)
            }
        );
        // A failed withdrawal changes nothing.
        assert_eq!(acct.balance(), cents(1_000));
        assert_eq!(acct.history().len(), 1);
    }

    #[test]
    fn withdrawing_everything_is_allowed() {
        let mut acct = account();
        acct.deposit(cents(500)).unwrap();
        acct.withdraw(cents(500)).unwrap();
        assert_eq!(acct.balance(), Money::ZERO);
    }

    #[test]
    fn zero_amounts_are_rejected() {
        let mut acct = account();
        assert_eq!(acct.deposit(Money::ZERO), Err(BankError::ZeroAmount));
        assert_eq!(acct.withdraw(Money::ZERO), Err(BankError::ZeroAmount));
        assert!(acct.history().is_empty());
    }

    #[test]
    fn deposits_cannot_overflow() {
        let mut acct = account();
        acct.deposit(cents(u64::MAX)).unwrap();
        assert!(!acct.can_receive(cents(1)));
        assert_eq!(acct.deposit(cents(1)), Err(BankError::Overflow));
        assert_eq!(acct.balance(), cents(u64::MAX));
    }

    #[test]
    fn largest_deposit_ignores_other_transactions() {
        let mut acct = account();
        acct.deposit(cents(500)).unwrap();
        acct.deposit(cents(9_000)).unwrap();
        acct.withdraw(cents(9_500)).unwrap();
        assert_eq!(acct.largest_deposit(), Some(cents(9_000)));
    }

    #[test]
    fn history_always_adds_up_to_the_balance() {
        let mut acct = account();
        acct.deposit(cents(10_000)).unwrap();
        acct.withdraw(cents(1_234)).unwrap();
        acct.receive(1002, cents(500)).unwrap();
        acct.send(1003, cents(2_000)).unwrap();
        let _ = acct.withdraw(cents(999_999)); // fails, must not be recorded

        let total: i128 = acct.history().iter().map(Transaction::signed_cents).sum();
        assert_eq!(total, i128::from(acct.balance().cents()));
        assert_eq!(acct.history().len(), 4);
    }

    #[test]
    fn transfers_remember_the_other_account() {
        let mut acct = account();
        acct.receive(1002, cents(700)).unwrap();
        acct.send(1003, cents(200)).unwrap();
        assert_eq!(
            acct.history(),
            [
                Transaction::TransferIn {
                    from: 1002,
                    amount: cents(700)
                },
                Transaction::TransferOut {
                    to: 1003,
                    amount: cents(200)
                },
            ]
        );
    }
}
