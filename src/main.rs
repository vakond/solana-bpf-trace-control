//! solana-bpf-trace-control main module.

#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

mod cli;
mod config;
mod error;
mod request;

fn main() {
    init_logger();
    if let Err(err) = execute(cli::application()) {
        eprintln!("Error: {:#}", err);
        std::process::exit(config::FAILURE);
    }
}

/// Initializes the logger.
fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();
}

use crate::error::Result;

/// Dispatches CLI commands.
fn execute(app: cli::Application) -> Result<()> {
    match app.cmd {
        None => show(),
        Some(cmd) => match cmd {
            cli::Command::Enable { value } => enable(value),
            cli::Command::Filter { value } => filter(value),
            cli::Command::Output { value } => output(value),
            cli::Command::Binary { value } => binary(value),
            cli::Command::MultipleFiles { value } => multiple_files(value),
            cli::Command::MaxThreads { value } => max_threads(value),
            cli::Command::MinProgram { value } => min_program(value),
        },
    }
}

use crate::config::{
    BINARY, ENABLE, FILTER, MAX_THREADS, MIN_PROGRAM, MULTIPLE_FILES, OUTPUT, SHOW,
};
use crate::request::request;

fn enable(value: Option<bool>) -> Result<()> {
    match value {
        None => request(ENABLE),
        Some(value) => request(&format!("{}={}", ENABLE, value)),
    }
}

fn filter(value: Option<String>) -> Result<()> {
    match value {
        None => request(FILTER),
        Some(value) => request(&format!("{}={}", FILTER, &value)),
    }
}

fn output(value: Option<String>) -> Result<()> {
    match value {
        None => request(OUTPUT),
        Some(value) => request(&format!("{}={}", OUTPUT, &value)),
    }
}

fn binary(value: Option<bool>) -> Result<()> {
    match value {
        None => request(BINARY),
        Some(value) => request(&format!("{}={}", BINARY, value)),
    }
}

fn multiple_files(value: Option<bool>) -> Result<()> {
    match value {
        None => request(MULTIPLE_FILES),
        Some(value) => request(&format!("{}={}", MULTIPLE_FILES, value)),
    }
}

fn max_threads(value: Option<usize>) -> Result<()> {
    match value {
        None => request(MAX_THREADS),
        Some(value) => request(&format!("{}={}", MAX_THREADS, value)),
    }
}

fn min_program(value: Option<usize>) -> Result<()> {
    match value {
        None => request(MIN_PROGRAM),
        Some(value) => request(&format!("{}={}", MIN_PROGRAM, value)),
    }
}

fn show() -> Result<()> {
    request(SHOW)
}
