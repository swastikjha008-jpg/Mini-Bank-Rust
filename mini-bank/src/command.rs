//! Turning a line of text into a [`Command`].
//!
//! **Concepts:** an enum with named fields and tuple-style data, `match` on a
//! tuple and on *slice patterns* (`[id, amount]`), `let ... else`, borrowing
//! (`&str` in, owned `String` only where the command must keep the text), and
//! `Result` with the `?` operator.

use crate::error::{BankError, Result};
use crate::money::Money;

/// Everything the user can ask the bank to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Open { owner: String },
    Deposit { id: u32, amount: Money },
    Withdraw { id: u32, amount: Money },
    Transfer { from: u32, to: u32, amount: Money },
    Balance { id: u32 },
    History { id: u32 },
    Search { text: String },
    List,
    Help,
    Quit,
}

impl Command {
    /// Parse one line such as `transfer 1001 1002 20.50`.
    ///
    /// The line is only *borrowed* (`&str`). The command keeps its own copy of
    /// the few pieces of text it needs, so it can outlive the line.
    ///
    /// ```
    /// use mini_bank::command::Command;
    ///
    /// assert_eq!(Command::parse("balance 1001").unwrap(), Command::Balance { id: 1001 });
    /// assert!(Command::parse("dance").is_err());
    /// ```
    pub fn parse(line: &str) -> Result<Command> {
        let words: Vec<&str> = line.split_whitespace().collect();

        // `let ... else`: split off the first word, or bail out on a blank line.
        let Some((verb, args)) = words.split_first() else {
            return Err(BankError::EmptyCommand);
        };

        // Match the command word and the *shape* of the arguments together.
        match (verb.to_ascii_lowercase().as_str(), args) {
            ("open", []) => Err(BankError::Usage("open <name>")),
            ("open", name_parts) => Ok(Command::Open {
                owner: name_parts.join(" "),
            }),

            ("deposit", [id, amount]) => Ok(Command::Deposit {
                id: parse_id(id)?,
                amount: amount.parse()?,
            }),
            ("deposit", _) => Err(BankError::Usage("deposit <id> <amount>")),

            ("withdraw", [id, amount]) => Ok(Command::Withdraw {
                id: parse_id(id)?,
                amount: amount.parse()?,
            }),
            ("withdraw", _) => Err(BankError::Usage("withdraw <id> <amount>")),

            ("transfer", [from, to, amount]) => Ok(Command::Transfer {
                from: parse_id(from)?,
                to: parse_id(to)?,
                amount: amount.parse()?,
            }),
            ("transfer", _) => Err(BankError::Usage("transfer <from-id> <to-id> <amount>")),

            ("balance", [id]) => Ok(Command::Balance { id: parse_id(id)? }),
            ("balance", _) => Err(BankError::Usage("balance <id>")),

            ("history", [id]) => Ok(Command::History { id: parse_id(id)? }),
            ("history", _) => Err(BankError::Usage("history <id>")),

            ("search", []) => Err(BankError::Usage("search <text>")),
            ("search", text_parts) => Ok(Command::Search {
                text: text_parts.join(" "),
            }),

            ("list", _) => Ok(Command::List),
            ("help" | "?", _) => Ok(Command::Help),
            ("quit" | "exit" | "q", _) => Ok(Command::Quit),

            (other, _) => Err(BankError::UnknownCommand(other.to_string())),
        }
    }
}

/// Parse an account id. A leading `#` is allowed (`#1001`).
fn parse_id(text: &str) -> Result<u32> {
    text.trim_start_matches('#')
        .parse()
        .map_err(|_| BankError::InvalidId(text.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cents(value: u64) -> Money {
        Money::from_cents(value)
    }

    fn parse(line: &str) -> Result<Command> {
        Command::parse(line)
    }

    #[test]
    fn parses_every_command() {
        assert_eq!(
            parse("open Asha").unwrap(),
            Command::Open {
                owner: "Asha".into()
            }
        );
        assert_eq!(
            parse("deposit 1001 50").unwrap(),
            Command::Deposit {
                id: 1001,
                amount: cents(5_000)
            }
        );
        assert_eq!(
            parse("withdraw 1001 12.5").unwrap(),
            Command::Withdraw {
                id: 1001,
                amount: cents(1_250)
            }
        );
        assert_eq!(
            parse("transfer 1001 1002 0.99").unwrap(),
            Command::Transfer {
                from: 1001,
                to: 1002,
                amount: cents(99)
            }
        );
        assert_eq!(
            parse("balance 1001").unwrap(),
            Command::Balance { id: 1001 }
        );
        assert_eq!(
            parse("history 1001").unwrap(),
            Command::History { id: 1001 }
        );
        assert_eq!(
            parse("search sha").unwrap(),
            Command::Search { text: "sha".into() }
        );
        assert_eq!(parse("list").unwrap(), Command::List);
        assert_eq!(parse("help").unwrap(), Command::Help);
        assert_eq!(parse("quit").unwrap(), Command::Quit);
    }

    #[test]
    fn aliases_case_and_spacing() {
        assert_eq!(parse("?").unwrap(), Command::Help);
        assert_eq!(parse("exit").unwrap(), Command::Quit);
        assert_eq!(parse("q").unwrap(), Command::Quit);
        assert_eq!(parse("  LIST  ").unwrap(), Command::List);
        assert_eq!(
            parse("DEPOSIT   #1001    5").unwrap(),
            Command::Deposit {
                id: 1001,
                amount: cents(500)
            }
        );
    }

    #[test]
    fn names_can_have_several_words() {
        assert_eq!(
            parse("open Asha  Kumari   Sharma").unwrap(),
            Command::Open {
                owner: "Asha Kumari Sharma".into()
            }
        );
    }

    #[test]
    fn name_keeps_its_capital_letters() {
        // Only the command word is lowercased, never the name.
        assert_eq!(
            parse("OPEN Ravi").unwrap(),
            Command::Open {
                owner: "Ravi".into()
            }
        );
    }

    #[test]
    fn wrong_argument_counts_show_usage() {
        assert_eq!(parse("open"), Err(BankError::Usage("open <name>")));
        assert_eq!(
            parse("deposit 1001"),
            Err(BankError::Usage("deposit <id> <amount>"))
        );
        assert_eq!(
            parse("deposit 1001 5 6"),
            Err(BankError::Usage("deposit <id> <amount>"))
        );
        assert_eq!(
            parse("transfer 1 2"),
            Err(BankError::Usage("transfer <from-id> <to-id> <amount>"))
        );
        assert_eq!(parse("balance"), Err(BankError::Usage("balance <id>")));
        assert_eq!(parse("history 1 2"), Err(BankError::Usage("history <id>")));
        assert_eq!(parse("search"), Err(BankError::Usage("search <text>")));
    }

    #[test]
    fn bad_values_are_reported() {
        assert_eq!(
            parse("deposit abc 5"),
            Err(BankError::InvalidId("abc".into()))
        );
        assert_eq!(
            parse("deposit 1001 lots"),
            Err(BankError::InvalidAmount("lots".into()))
        );
        assert_eq!(
            parse("deposit 1001 -5"),
            Err(BankError::InvalidAmount("-5".into()))
        );
    }

    #[test]
    fn blank_and_unknown_lines() {
        assert_eq!(parse(""), Err(BankError::EmptyCommand));
        assert_eq!(parse("   "), Err(BankError::EmptyCommand));
        assert_eq!(
            parse("dance now"),
            Err(BankError::UnknownCommand("dance".into()))
        );
    }
}
