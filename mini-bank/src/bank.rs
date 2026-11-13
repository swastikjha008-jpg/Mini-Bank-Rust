//! The bank: a collection of accounts.
//!
//! **Concepts:** borrowing in practice. Methods return references into the
//! bank (`&Account`, `&mut Account`), lookups return `Option` and the checked
//! versions turn that into `Result`. `transfer` shows why Rust allows only one
//! mutable borrow at a time, and how to work with that rule.

use crate::account::Account;
use crate::error::{BankError, Result};
use crate::money::Money;

/// The first account number handed out.
const FIRST_ID: u32 = 1001;

#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Account>,
    next_id: u32,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: Vec::new(),
            next_id: FIRST_ID,
        }
    }

    /// Open an account and return its id.
    pub fn open(&mut self, owner: &str) -> Result<u32> {
        let owner = owner.trim();
        if owner.is_empty() {
            return Err(BankError::EmptyName);
        }

        let id = self.next_id;
        self.next_id += 1;
        self.accounts.push(Account::new(id, owner));
        Ok(id)
    }

    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }

    // ---- Lookups ------------------------------------------------------------

    /// Borrow an account read-only. `None` if the id does not exist.
    pub fn find(&self, id: u32) -> Option<&Account> {
        self.accounts.iter().find(|account| account.id() == id)
    }

    /// Borrow an account for changing. Private: outside code goes through the
    /// bank's methods, so the rules are always applied.
    fn find_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|account| account.id() == id)
    }

    /// Like [`find`](Self::find), but a missing account is an error.
    pub fn account(&self, id: u32) -> Result<&Account> {
        self.find(id).ok_or(BankError::AccountNotFound(id))
    }

    fn account_mut(&mut self, id: u32) -> Result<&mut Account> {
        self.find_mut(id).ok_or(BankError::AccountNotFound(id))
    }

    /// Accounts whose owner contains `text` (ignoring case).
    ///
    /// The returned references point *into* the bank, so Rust makes sure the
    /// bank cannot be changed or dropped while they are still in use.
    pub fn find_by_owner(&self, text: &str) -> Vec<&Account> {
        let needle = text.trim().to_lowercase();
        self.accounts
            .iter()
            .filter(|account| account.owner().to_lowercase().contains(&needle))
            .collect()
    }

    /// The account with the highest balance, or `None` for an empty bank.
    pub fn richest(&self) -> Option<&Account> {
        self.accounts.iter().max_by_key(|account| account.balance())
    }

    // ---- Changing money -----------------------------------------------------

    pub fn deposit(&mut self, id: u32, amount: Money) -> Result<()> {
        self.account_mut(id)?.deposit(amount)
    }

    pub fn withdraw(&mut self, id: u32, amount: Money) -> Result<()> {
        self.account_mut(id)?.withdraw(amount)
    }

    /// Move money between two accounts, all or nothing.
    ///
    /// We cannot hold `&mut` to both accounts at once, because both live in
    /// the same `Vec` and Rust allows only one exclusive borrow at a time.
    /// So we (1) check everything using shared borrows, then (2) borrow each
    /// account mutably, one after the other.
    pub fn transfer(&mut self, from: u32, to: u32, amount: Money) -> Result<()> {
        if from == to {
            return Err(BankError::SameAccount);
        }
        if amount.is_zero() {
            return Err(BankError::ZeroAmount);
        }

        // Step 1: look, but do not touch.
        self.account(from)?;
        if !self.account(to)?.can_receive(amount) {
            return Err(BankError::Overflow);
        }

        // Step 2: change. `send` fails cleanly (and changes nothing) when the
        // source lacks funds, so the second borrow is never reached.
        self.account_mut(from)?.send(to, amount)?;
        self.account_mut(to)?.receive(from, amount)
    }
}

impl Default for Bank {
    fn default() -> Self {
        Bank::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cents(value: u64) -> Money {
        Money::from_cents(value)
    }

    /// Two accounts: 1001 (Asha, 100.00) and 1002 (Ravi, 0.00).
    fn bank() -> Bank {
        let mut bank = Bank::new();
        bank.open("Asha Sharma").unwrap();
        bank.open("Ravi Kumar").unwrap();
        bank.deposit(1001, cents(10_000)).unwrap();
        bank
    }

    fn balance(bank: &Bank, id: u32) -> Money {
        bank.account(id).unwrap().balance()
    }

    #[test]
    fn ids_start_at_1001_and_count_up() {
        let mut bank = Bank::default();
        assert_eq!(bank.open("A").unwrap(), 1001);
        assert_eq!(bank.open("B").unwrap(), 1002);
        assert_eq!(bank.accounts().len(), 2);
    }

    #[test]
    fn names_are_trimmed_and_required() {
        let mut bank = Bank::new();
        let id = bank.open("  Asha  ").unwrap();
        assert_eq!(bank.account(id).unwrap().owner(), "Asha");
        assert_eq!(bank.open("   "), Err(BankError::EmptyName));
        assert_eq!(bank.accounts().len(), 1);
    }

    #[test]
    fn find_returns_option_and_account_returns_result() {
        let bank = bank();
        assert!(bank.find(1001).is_some());
        assert!(bank.find(9999).is_none());
        assert_eq!(bank.account(9999), Err(BankError::AccountNotFound(9999)));
    }

    #[test]
    fn search_ignores_case_and_matches_parts() {
        let bank = bank();
        let found = bank.find_by_owner("SHARMA");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id(), 1001);
        assert_eq!(bank.find_by_owner("a").len(), 2);
        assert!(bank.find_by_owner("zzz").is_empty());
    }

    #[test]
    fn richest_is_an_option() {
        assert!(Bank::new().richest().is_none());
        assert_eq!(bank().richest().map(Account::id), Some(1001));
    }

    #[test]
    fn deposit_and_withdraw_by_id() {
        let mut bank = bank();
        bank.deposit(1002, cents(2_500)).unwrap();
        bank.withdraw(1002, cents(500)).unwrap();
        assert_eq!(balance(&bank, 1002), cents(2_000));
        assert_eq!(
            bank.deposit(4242, cents(100)),
            Err(BankError::AccountNotFound(4242))
        );
    }

    #[test]
    fn transfer_moves_money_and_records_both_sides() {
        let mut bank = bank();
        bank.transfer(1001, 1002, cents(2_500)).unwrap();

        assert_eq!(balance(&bank, 1001), cents(7_500));
        assert_eq!(balance(&bank, 1002), cents(2_500));
        assert_eq!(
            bank.account(1001).unwrap().history().last().copied(),
            Some(crate::transaction::Transaction::TransferOut {
                to: 1002,
                amount: cents(2_500)
            })
        );
        assert_eq!(
            bank.account(1002).unwrap().history().last().copied(),
            Some(crate::transaction::Transaction::TransferIn {
                from: 1001,
                amount: cents(2_500)
            })
        );
    }

    #[test]
    fn failed_transfers_change_nothing() {
        let mut bank = bank();

        assert!(matches!(
            bank.transfer(1001, 1002, cents(10_001)),
            Err(BankError::InsufficientFunds { .. })
        ));
        assert_eq!(
            bank.transfer(1001, 7777, cents(100)),
            Err(BankError::AccountNotFound(7777))
        );
        assert_eq!(
            bank.transfer(7777, 1001, cents(100)),
            Err(BankError::AccountNotFound(7777))
        );
        assert_eq!(
            bank.transfer(1001, 1001, cents(100)),
            Err(BankError::SameAccount)
        );
        assert_eq!(
            bank.transfer(1001, 1002, Money::ZERO),
            Err(BankError::ZeroAmount)
        );

        assert_eq!(balance(&bank, 1001), cents(10_000));
        assert_eq!(balance(&bank, 1002), Money::ZERO);
        assert_eq!(bank.account(1001).unwrap().history().len(), 1);
        assert!(bank.account(1002).unwrap().history().is_empty());
    }

    #[test]
    fn transfer_that_would_overflow_the_target_is_refused() {
        let mut bank = bank();
        bank.deposit(1002, cents(u64::MAX - 50)).unwrap();
        assert_eq!(
            bank.transfer(1001, 1002, cents(100)),
            Err(BankError::Overflow)
        );
        assert_eq!(balance(&bank, 1001), cents(10_000));
    }

    #[test]
    fn money_is_conserved_across_transfers() {
        let mut bank = bank();
        bank.transfer(1001, 1002, cents(3_333)).unwrap();
        bank.transfer(1002, 1001, cents(1_000)).unwrap();

        let total: u64 = bank.accounts().iter().map(|a| a.balance().cents()).sum();
        assert_eq!(total, 10_000);
    }
}
