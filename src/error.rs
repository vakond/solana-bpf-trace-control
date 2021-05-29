//! solana-bpf-trace-control error module.

use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Variable '{0}' does not exist. It should contain a TCP port")]
    NoEnvVar(String),

    #[error("TCP connect: {0}: {1}")]
    Connect(#[source] io::Error, String),

    #[error("Input/output: {0}")]
    Io(#[from] io::Error),
}

/// Represents results.
pub type Result<T> = std::result::Result<T, Error>;
