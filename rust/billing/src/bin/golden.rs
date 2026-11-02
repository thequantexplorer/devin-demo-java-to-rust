//! `golden <input.txt>`: prints the same output as `com.acme.billing.GoldenCli`.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("usage: golden <input.txt>");
        return ExitCode::from(2);
    }
    let bytes = match std::fs::read(&args[0]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: {e}", args[0]);
            return ExitCode::FAILURE;
        }
    };
    match acme_billing::golden_cli::run(&String::from_utf8_lossy(&bytes)) {
        Ok(out) => {
            print!("{out}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
