//! End-to-end tests: start the real program, type commands into it, and check
//! what it prints.

use std::io::Write;
use std::process::{Command, Stdio};

struct Session {
    stdout: String,
    stderr: String,
    success: bool,
}

/// Run the binary with `input` as everything the user types.
fn run(input: &str) -> Session {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mini-bank"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start mini-bank");

    child
        .stdin
        .take()
        .expect("stdin not piped")
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    let output = child.wait_with_output().expect("failed to wait");
    Session {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        success: output.status.success(),
    }
}

#[test]
fn full_session() {
    let session = run("\
open Asha Sharma
open Ravi
deposit 1001 100
withdraw 1001 30.50
transfer 1001 1002 20
balance 1001
history 1001
list
quit
");

    assert!(session.success);
    assert!(session.stderr.is_empty(), "stderr: {}", session.stderr);

    let out = &session.stdout;
    assert!(out.contains("Opened account #1001 for Asha Sharma"));
    assert!(out.contains("Opened account #1002 for Ravi"));
    assert!(out.contains("Deposited 100.00 into #1001. New balance: 100.00"));
    assert!(out.contains("Withdrew 30.50 from #1001. New balance: 69.50"));
    assert!(out.contains("Transferred 20.00 from #1001 to #1002"));
    assert!(out.contains("Asha Sharma (#1001): 49.50"));
    assert!(out.contains("1. deposit       +100.00"));
    assert!(out.contains("3. transfer out  -20.00  to #1002"));
    assert!(out.contains("Highest balance: Asha Sharma (#1001)"));
    assert!(out.trim_end().ends_with("Goodbye!"));
}

#[test]
fn errors_go_to_stderr_and_the_session_continues() {
    let session = run("\
open Asha
withdraw 1001 5
deposit 1001 abc
balance 9999
dance
open
deposit 1001 10
");

    assert!(session.success, "errors must not crash the program");

    let err = &session.stderr;
    assert!(err.contains("error: insufficient funds: need 5.00 but only 0.00 available"));
    assert!(err.contains("error: 'abc' is not a valid amount"));
    assert!(err.contains("error: no account with id 9999"));
    assert!(err.contains("error: unknown command 'dance'"));
    assert!(err.contains("error: usage: open <name>"));

    // The last command still ran after all those mistakes.
    assert!(session
        .stdout
        .contains("Deposited 10.00 into #1001. New balance: 10.00"));
}

#[test]
fn failed_transfer_changes_nothing() {
    let session = run("\
open Asha
open Ravi
deposit 1001 10
transfer 1001 1002 50
transfer 1001 9999 1
transfer 1001 1001 1
balance 1001
balance 1002
");

    assert!(session.stderr.contains("insufficient funds"));
    assert!(session.stderr.contains("no account with id 9999"));
    assert!(session
        .stderr
        .contains("cannot transfer to the same account"));
    assert!(session.stdout.contains("Asha (#1001): 10.00"));
    assert!(session.stdout.contains("Ravi (#1002): 0.00"));
}

#[test]
fn blank_lines_are_ignored_and_input_may_just_end() {
    let session = run("\n\n   \nopen Asha\n\nlist");
    assert!(session.success);
    assert!(session.stderr.is_empty());
    assert!(session.stdout.contains("1001  Asha"));
}

#[test]
fn no_prompt_or_banner_when_piped() {
    let session = run("help\nquit\n");
    assert!(!session.stdout.contains("bank>"));
    assert!(!session.stdout.contains("Type `help` for commands"));
    assert!(session.stdout.starts_with("Commands:"));
}

#[test]
fn search_and_empty_states() {
    let session = run("\
list
history 1001
open Asha Sharma
search SHARMA
search nobody
");

    assert!(session.stdout.contains("No accounts yet. Try: open Asha"));
    assert!(session.stdout.contains("#1001  Asha Sharma"));
    assert!(session
        .stdout
        .contains("No account owner matches 'nobody'."));
    assert!(session.stderr.contains("no account with id 1001"));
}

#[test]
fn commands_ignore_case_but_names_keep_theirs() {
    let session = run("OPEN Ravi Kumar\nDEPOSIT #1001 5\nBALANCE 1001\n");
    assert!(session
        .stdout
        .contains("Opened account #1001 for Ravi Kumar"));
    assert!(session.stdout.contains("Ravi Kumar (#1001): 5.00"));
}
