//! solana-bpf-trace-control request module.

use crate::config::PORT;
use crate::error::{Error, Result};
use std::io::{Read, Write};
use std::net::TcpStream;

/// Sends a command to the TCP server and prints the response.
pub fn request(command: &str) -> Result<()> {
    let port = std::env::var(PORT).unwrap_or_default();
    if port.is_empty() {
        return Err(Error::NoEnvVar(PORT.into()));
    }

    let addr = format!("127.0.0.1:{}", &port);
    let mut stream = TcpStream::connect(addr.clone()).map_err(|e| Error::Connect(e, addr))?;
    stream.write_all(command.as_bytes())?;

    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf)?;
    let resp = String::from_utf8_lossy(&buf[0..bytes_read]);
    println!("{}", &resp);

    Ok(())
}
