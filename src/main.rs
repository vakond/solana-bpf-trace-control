//! solana-bpf-trace-control main module.

#![forbid(unsafe_code)]
#![deny(warnings)]

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
            cli::Command::Multiple { value } => multiple(value),
            cli::Command::Output { value } => output(value),
        },
    }
}

use crate::config::{ENABLE, FILTER, MULTIPLE, OUTPUT, SHOW};
use crate::request::request;

fn enable(value: Option<bool>) -> Result<()> {
    match value {
        None => request(ENABLE),
        Some(value) => request(&format!("{}={}", ENABLE, &value)),
    }
}

fn filter(value: Option<String>) -> Result<()> {
    match value {
        None => request(FILTER),
        Some(value) => request(&format!("{}={}", FILTER, &value)),
    }
}

fn multiple(value: Option<bool>) -> Result<()> {
    match value {
        None => request(MULTIPLE),
        Some(value) => request(&format!("{}={}", MULTIPLE, &value)),
    }
}

fn output(value: Option<String>) -> Result<()> {
    match value {
        None => request(OUTPUT),
        Some(value) => request(&format!("{}={}", OUTPUT, &value)),
    }
}

fn show() -> Result<()> {
    request(SHOW)
}
