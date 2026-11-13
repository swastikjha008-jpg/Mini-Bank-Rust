# 🏦 mini-bank

A tiny bank simulator for the terminal, built to practise the core ideas of Rust with nothing but the standard library.

[![CI](https://img.shields.io/github/actions/workflow/status/swastikjha008-jpg/mini-bank/ci.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/swastikjha008-jpg/mini-bank/actions/workflows/ci.yml)
![Tests](https://img.shields.io/badge/tests-55%20passing-22c55e?style=flat-square&logo=checkmarx&logoColor=white)
![Clippy](https://img.shields.io/badge/clippy-clean-0ea5e9?style=flat-square&logo=rust&logoColor=white)
![Dependencies](https://img.shields.io/badge/dependencies-0-a855f7?style=flat-square)
[![License: MIT](https://img.shields.io/badge/license-MIT-f59e0b?style=flat-square)](LICENSE)

---

## 🧰 Tech Stack

Language and tools

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=F74C00)
![Cargo](https://img.shields.io/badge/Cargo-7c2d12?style=for-the-badge&logo=rust&logoColor=fdba74)
![Std Library](https://img.shields.io/badge/Std_Library_Only-1e3a8a?style=for-the-badge&logo=rust&logoColor=93c5fd)
![Terminal](https://img.shields.io/badge/Terminal_REPL-0f172a?style=for-the-badge&logo=gnometerminal&logoColor=4ade80)

Quality and automation

![rustfmt](https://img.shields.io/badge/rustfmt-4c1d95?style=for-the-badge&logo=rust&logoColor=c4b5fd)
![Clippy](https://img.shields.io/badge/Clippy-14532d?style=for-the-badge&logo=rust&logoColor=86efac)
![GitHub Actions](https://img.shields.io/badge/GitHub_Actions-0f172a?style=for-the-badge&logo=githubactions&logoColor=2088FF)
![Git](https://img.shields.io/badge/Git-7f1d1d?style=for-the-badge&logo=git&logoColor=fca5a5)

Platforms

![Windows](https://img.shields.io/badge/Windows-075985?style=for-the-badge&logo=windows&logoColor=7dd3fc)
![macOS](https://img.shields.io/badge/macOS-27272a?style=for-the-badge&logo=apple&logoColor=e4e4e7)
![Linux](https://img.shields.io/badge/Linux-713f12?style=for-the-badge&logo=linux&logoColor=fde047)

---

## 🦀 Rust Topics Covered

The six ideas this project was built around

![Borrowing](https://img.shields.io/badge/Borrowing_and_References-f97316?style=for-the-badge&logo=rust&logoColor=white)
![Structs](https://img.shields.io/badge/Structs-3b82f6?style=for-the-badge&logo=rust&logoColor=white)
![Impl](https://img.shields.io/badge/impl_Blocks-06b6d4?style=for-the-badge&logo=rust&logoColor=white)
![Enums](https://img.shields.io/badge/Enums-a855f7?style=for-the-badge&logo=rust&logoColor=white)
![Pattern Matching](https://img.shields.io/badge/Pattern_Matching-ec4899?style=for-the-badge&logo=rust&logoColor=white)
![Enums with values](https://img.shields.io/badge/Enums_with_Values-8b5cf6?style=for-the-badge&logo=rust&logoColor=white)
![Result](https://img.shields.io/badge/Result-ef4444?style=for-the-badge&logo=rust&logoColor=white)
![Option](https://img.shields.io/badge/Option-22c55e?style=for-the-badge&logo=rust&logoColor=white)

Extra topics you will meet along the way

![Traits](https://img.shields.io/badge/Traits-0ea5e9?style=flat-square&logo=rust&logoColor=white)
![Display](https://img.shields.io/badge/Display-14b8a6?style=flat-square&logo=rust&logoColor=white)
![FromStr](https://img.shields.io/badge/FromStr-84cc16?style=flat-square&logo=rust&logoColor=white)
![Question mark](https://img.shields.io/badge/The_%3F_Operator-eab308?style=flat-square&logo=rust&logoColor=white)
![Slice patterns](https://img.shields.io/badge/Slice_Patterns-f43f5e?style=flat-square&logo=rust&logoColor=white)
![let else](https://img.shields.io/badge/let--else-d946ef?style=flat-square&logo=rust&logoColor=white)
![Iterators](https://img.shields.io/badge/Iterators_and_Closures-6366f1?style=flat-square&logo=rust&logoColor=white)
![Modules](https://img.shields.io/badge/Modules_and_Privacy-f59e0b?style=flat-square&logo=rust&logoColor=white)
![Constants](https://img.shields.io/badge/Associated_Constants-10b981?style=flat-square&logo=rust&logoColor=white)
![Testing](https://img.shields.io/badge/Unit_Doc_and_Integration_Tests-64748b?style=flat-square&logo=rust&logoColor=white)

---

## 🌟 What is it?

mini-bank is an interactive program where you open accounts, deposit and withdraw money, transfer between accounts and read a statement. There are no outside crates. Everything is written with Rust's standard library, so you can read all of the code and see how each piece works.

```text
$ cargo run
Mini Bank. Type `help` for commands, `quit` to leave.
bank> open Asha Sharma
Opened account #1001 for Asha Sharma
bank> open Ravi Kumar
Opened account #1002 for Ravi Kumar
bank> deposit 1001 100
Deposited 100.00 into #1001. New balance: 100.00
bank> transfer 1001 1002 20
Transferred 20.00 from #1001 to #1002
bank> withdraw 1002 500
error: insufficient funds: need 500.00 but only 20.00 available
bank> list
ID    OWNER                BALANCE
1001  Asha Sharma            80.00
1002  Ravi Kumar             20.00

Highest balance: Asha Sharma (#1001)
bank> quit
Goodbye!
```

Mistakes never crash the program. A bad command prints a clear error and you carry on.

## ⚡ Quick Start

You need Rust 1.75 or newer from [rustup.rs](https://rustup.rs). On Windows, also install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **Desktop development with C++** workload, because Rust needs Microsoft's linker.

```bash
git clone https://github.com/swastikjha008-jpg/mini-bank.git
cd mini-bank
cargo run
```

Type `help` inside the program to see every command. You can also feed it a script:

```bash
printf 'open Asha\ndeposit 1001 50\nbalance 1001\n' | cargo run -q
```

When input is piped in, the banner and prompt are hidden so the output stays clean.

## 📖 Commands

`open <name>` opens a new account. Names can have several words, and ids start at 1001.

`deposit <id> <amount>` adds money and `withdraw <id> <amount>` takes it out. Amounts look like `25` or `25.50`.

`transfer <from> <to> <amount>` moves money between two accounts. It either fully works or changes nothing.

`balance <id>` shows an account and its latest activity, and `history <id>` lists every transaction.

`search <text>` finds accounts by owner name, ignoring case, and `list` shows all accounts.

`help` shows this list in the program, and `quit` leaves. Command words are not case-sensitive, and an id can be written as `1001` or `#1001`.

## 🧠 How it works

The program is a loop with four steps. It **reads** a line from the keyboard, **parses** the text into a `Command`, **runs** that command against the `Bank`, and **prints** the result. Then it waits for the next line.

Parsing lives in `command.rs`, which turns text like `deposit 1001 50` into `Command::Deposit { id: 1001, amount: 50.00 }`. Running lives in `app.rs`, which hands the command to the bank. The bank finds the right account, the account changes its own balance, and every change is written into that account's history.

Three rules keep the money safe. Money is stored as whole cents, so there are no rounding mistakes. A balance can never go negative. And every action is all-or-nothing: a failed transfer leaves both accounts exactly as they were.

## 📚 Rust topics, and where to find them

Each paragraph below explains one idea and points to the file where you can see it in action. The source files have comments that mention the same things.

### 🟠 Borrowing and references

Rust lets you borrow a value instead of taking it, so nothing needs to be copied. `&` is a read-only borrow and `&mut` is an exclusive, changeable one, and Rust never lets the two overlap. In `account.rs`, methods like `owner()` and `history()` return references into the account. In `bank.rs`, `transfer` is the interesting one: it cannot hold two `&mut` borrows into the same list at once, so it first checks everything with shared borrows and then borrows each account mutably, one after the other.

### 🔵 Structs and implementing structs

A struct bundles related data, and an `impl` block gives it behavior. `Account` in `account.rs` has private fields and a small set of methods that are the only way to change its balance, so the rules cannot be bypassed. `Bank` in `bank.rs` owns a list of accounts. `Money` in `money.rs` is a tuple struct that stores cents as a whole number, with an associated constant `Money::ZERO`.

### 🟣 Enums and pattern matching

An enum is a value that can be exactly one of several options, and `match` forces you to handle every option. `Command` in `command.rs` lists everything a user can ask for, and `parse` uses `match` on the command word together with the shape of the arguments, using slice patterns like `[id, amount]`. If you add a new variant later, the compiler points at every `match` that needs updating.

### 🩷 Enums with values

Variants can carry data. `Transaction` in `transaction.rs` is the clearest example: a `Deposit` holds an amount, while a `TransferOut` holds the other account's id and an amount. `BankError` in `error.rs` does the same, so `InsufficientFunds { needed, available }` can print a message with real numbers in it.

### 🟢 Implementing enums

Enums get methods too. `Transaction` has `amount()`, `is_credit()`, `signed_cents()` and `describe()`, and each one is a `match` over the variants. `amount()` handles four variants in a single arm with an or-pattern. `BankError` implements `Display`, so every error turns into a readable sentence.

### 🔴 Result and Option

Both are just enums from the standard library. `Option` means "a value, or nothing": `Bank::find` returns `Some(account)` or `None`, and `Account::last_transaction` is `None` for a brand-new account. `Result` means "success, or an error": `withdraw` returns `Ok(())` or `Err(BankError::InsufficientFunds { .. })`. The `?` operator passes an error up to the caller in one character, and `ok_or` turns an `Option` into a `Result`. You can see the whole chain in `Bank::deposit`, which is a single line.

## 🏗️ How the code is organised

The library does the work and `main.rs` only reads lines and prints replies. `error.rs` holds the error enum and `money.rs` holds the cents type. `transaction.rs` describes one history entry, `account.rs` is a single account, and `bank.rs` manages many of them. `command.rs` turns text into a command and `app.rs` runs a command and returns what to show.

The program logic returns text instead of printing it, so every behavior can be tested without a terminal.

## 🧪 Testing

```bash
cargo test
```

There are 55 tests: 45 unit tests next to the code, 7 end-to-end tests that start the real program and type commands into it, and 3 examples inside the documentation. One test even checks that an account's history always adds up to its balance. Before committing, run the same checks CI does:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## 💡 Ideas to try next

Add an `interest <id> <percent>` command and see how the compiler helps you handle every case. Save the bank to a file so it survives restarts, which is a good first use of `serde`. Try adding a `Frozen` state to `Account` as an enum and watch which `match` expressions the compiler asks you to update.

## ❓ Troubleshooting

**`linker link.exe not found` on Windows.** Install the Visual Studio Build Tools with the C++ workload, then fully restart your terminal or editor.

**The build fails with a version error.** Your Rust is too old. Run `rustup update`.

**My accounts are gone after quitting.** That is expected. The bank lives in memory only, which keeps the code simple to learn from.

## 🤝 Contributing

Ideas and pull requests are welcome. Fork the repo, create a branch, run the three checks above, and open a pull request.

## 📄 License

Released under the [MIT License](LICENSE).
