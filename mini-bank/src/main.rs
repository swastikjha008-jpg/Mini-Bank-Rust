use std::io::{self, BufRead, IsTerminal, Write};

use mini_bank::app::{execute, Reply};
use mini_bank::bank::Bank;
use mini_bank::command::Command;

fn main() {
    // Only show the banner and prompt when a person is typing. When input is
    // piped in (`echo "list" | mini-bank`), stay quiet so output is clean.
    let interactive = io::stdin().is_terminal();
    let mut bank = Bank::new();

    if interactive {
        println!("Mini Bank. Type `help` for commands, `quit` to leave.");
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        if interactive {
            print!("bank> ");
            // `print!` does not flush by itself, so the prompt could stay hidden.
            let _ = io::stdout().flush();
        }

        // End of input (Ctrl-D / Ctrl-Z, or a finished pipe) or a read error.
        let Some(Ok(line)) = lines.next() else {
            break;
        };
        if line.trim().is_empty() {
            continue;
        }

        // Parse, then run. An error from either step lands in the `Err` arm.
        match Command::parse(&line).and_then(|command| execute(&mut bank, command)) {
            Ok(Reply::Text(text)) => println!("{text}"),
            Ok(Reply::Exit(text)) => {
                println!("{text}");
                break;
            }
            Err(error) => eprintln!("error: {error}"),
        }
    }
}
