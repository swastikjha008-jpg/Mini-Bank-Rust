//! Running a [`Command`] against a [`Bank`] and producing text to show.
//!
//! **Concepts:** `match` on an enum (one arm per command), passing `&mut Bank`
//! into a function, `if let` for `Option`, and an enum (`Reply`) as a return
//! value. The function returns text instead of printing, so it is easy to test.

use crate::bank::Bank;
use crate::command::Command;
use crate::error::Result;

/// What the program should do after a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    /// Show this text and keep going.
    Text(String),
    /// Show this text and stop.
    Exit(String),
}

pub const HELP: &str = "\
Commands:
  open <name>                      open a new account
  deposit <id> <amount>            add money
  withdraw <id> <amount>           take money out
  transfer <from> <to> <amount>    move money between accounts
  balance <id>                     show an account
  history <id>                     show every transaction
  search <text>                    find accounts by owner name
  list                             show all accounts
  help                             show this message
  quit                             leave

Amounts look like 25 or 25.50. Account ids start at 1001.";

/// Carry out `command`. The bank is borrowed mutably, because most commands
/// change it, and returned to the caller untouched in shape afterwards.
pub fn execute(bank: &mut Bank, command: Command) -> Result<Reply> {
    let text = match command {
        Command::Open { owner } => {
            let id = bank.open(&owner)?;
            format!("Opened account #{id} for {}", owner.trim())
        }

        Command::Deposit { id, amount } => {
            bank.deposit(id, amount)?;
            let balance = bank.account(id)?.balance();
            format!("Deposited {amount} into #{id}. New balance: {balance}")
        }

        Command::Withdraw { id, amount } => {
            bank.withdraw(id, amount)?;
            let balance = bank.account(id)?.balance();
            format!("Withdrew {amount} from #{id}. New balance: {balance}")
        }

        Command::Transfer { from, to, amount } => {
            bank.transfer(from, to, amount)?;
            format!("Transferred {amount} from #{from} to #{to}")
        }

        Command::Balance { id } => {
            let account = bank.account(id)?;
            let mut text = format!(
                "{} (#{}): {}",
                account.owner(),
                account.id(),
                account.balance()
            );
            // `last_transaction` is an Option: there may be nothing to show.
            if let Some(last) = account.last_transaction() {
                text.push_str(&format!("\n  last activity: {}", last.describe()));
            }
            text
        }

        Command::History { id } => {
            let account = bank.account(id)?;
            if account.history().is_empty() {
                format!("No transactions yet for {} (#{id}).", account.owner())
            } else {
                let lines: Vec<String> = account
                    .history()
                    .iter()
                    .enumerate()
                    .map(|(index, transaction)| {
                        format!("{:>3}. {}", index + 1, transaction.describe())
                    })
                    .collect();
                format!(
                    "History for {} (#{id}):\n{}",
                    account.owner(),
                    lines.join("\n")
                )
            }
        }

        Command::Search { text } => {
            let matches = bank.find_by_owner(&text);
            if matches.is_empty() {
                format!("No account owner matches '{}'.", text.trim())
            } else {
                matches
                    .iter()
                    .map(|account| {
                        format!(
                            "#{}  {:<16}{:>12}",
                            account.id(),
                            account.owner(),
                            account.balance()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }

        Command::List => {
            if bank.accounts().is_empty() {
                "No accounts yet. Try: open Asha".to_string()
            } else {
                let mut lines = vec![format!("{:<6}{:<16}{:>12}", "ID", "OWNER", "BALANCE")];
                for account in bank.accounts() {
                    lines.push(format!(
                        "{:<6}{:<16}{:>12}",
                        account.id(),
                        account.owner(),
                        account.balance()
                    ));
                }
                if let Some(richest) = bank.richest() {
                    lines.push(format!(
                        "\nHighest balance: {} (#{})",
                        richest.owner(),
                        richest.id()
                    ));
                }
                lines.join("\n")
            }
        }

        Command::Help => HELP.to_string(),

        Command::Quit => return Ok(Reply::Exit("Goodbye!".to_string())),
    };

    Ok(Reply::Text(text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BankError;

    /// Parse and run a line, returning the reply text.
    fn run(bank: &mut Bank, line: &str) -> String {
        match execute(bank, Command::parse(line).unwrap()).unwrap() {
            Reply::Text(text) | Reply::Exit(text) => text,
        }
    }

    fn run_err(bank: &mut Bank, line: &str) -> BankError {
        execute(bank, Command::parse(line).unwrap()).unwrap_err()
    }

    #[test]
    fn open_deposit_withdraw_flow() {
        let mut bank = Bank::new();
        assert_eq!(
            run(&mut bank, "open Asha Sharma"),
            "Opened account #1001 for Asha Sharma"
        );
        assert_eq!(
            run(&mut bank, "deposit 1001 100"),
            "Deposited 100.00 into #1001. New balance: 100.00"
        );
        assert_eq!(
            run(&mut bank, "withdraw 1001 30.5"),
            "Withdrew 30.50 from #1001. New balance: 69.50"
        );
    }

    #[test]
    fn transfer_and_balance() {
        let mut bank = Bank::new();
        run(&mut bank, "open Asha");
        run(&mut bank, "open Ravi");
        run(&mut bank, "deposit 1001 100");

        assert_eq!(
            run(&mut bank, "transfer 1001 1002 20"),
            "Transferred 20.00 from #1001 to #1002"
        );
        assert_eq!(
            run(&mut bank, "balance 1001"),
            "Asha (#1001): 80.00\n  last activity: transfer out  -20.00  to #1002"
        );
        assert_eq!(
            run(&mut bank, "balance 1002"),
            "Ravi (#1002): 20.00\n  last activity: transfer in   +20.00  from #1001"
        );
    }

    #[test]
    fn balance_without_activity_has_no_last_line() {
        let mut bank = Bank::new();
        run(&mut bank, "open Asha");
        assert_eq!(run(&mut bank, "balance 1001"), "Asha (#1001): 0.00");
    }

    #[test]
    fn history_is_numbered() {
        let mut bank = Bank::new();
        run(&mut bank, "open Asha");
        assert_eq!(
            run(&mut bank, "history 1001"),
            "No transactions yet for Asha (#1001)."
        );

        run(&mut bank, "deposit 1001 50");
        run(&mut bank, "withdraw 1001 5");
        assert_eq!(
            run(&mut bank, "history 1001"),
            "History for Asha (#1001):\n  1. deposit       +50.00\n  2. withdrawal    -5.00"
        );
    }

    #[test]
    fn list_and_search() {
        let mut bank = Bank::new();
        assert_eq!(run(&mut bank, "list"), "No accounts yet. Try: open Asha");

        run(&mut bank, "open Asha");
        run(&mut bank, "open Ravi");
        run(&mut bank, "deposit 1002 7.5");

        let list = run(&mut bank, "list");
        assert!(list.contains("ID"));
        assert!(list.contains("1001  Asha"));
        assert!(list.contains("7.50"));
        assert!(list.contains("Highest balance: Ravi (#1002)"));

        assert!(run(&mut bank, "search rav").contains("Ravi"));
        assert_eq!(
            run(&mut bank, "search zzz"),
            "No account owner matches 'zzz'."
        );
    }

    #[test]
    fn errors_bubble_up() {
        let mut bank = Bank::new();
        run(&mut bank, "open Asha");
        assert!(matches!(
            run_err(&mut bank, "withdraw 1001 5"),
            BankError::InsufficientFunds { .. }
        ));
        assert_eq!(
            run_err(&mut bank, "balance 9999"),
            BankError::AccountNotFound(9999)
        );
        assert_eq!(run_err(&mut bank, "deposit 1001 0"), BankError::ZeroAmount);
    }

    #[test]
    fn quit_and_help() {
        let mut bank = Bank::new();
        assert_eq!(
            execute(&mut bank, Command::Quit).unwrap(),
            Reply::Exit("Goodbye!".to_string())
        );
        assert!(run(&mut bank, "help").starts_with("Commands:"));
    }
}
