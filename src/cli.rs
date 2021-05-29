//! solana-bpf-trace-control command line interface definition.

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Solana BPF trace control program")]
pub struct Application {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Enables or disables BPF tracing")]
    Enable { value: Option<bool> },
    #[structopt(about = "Applies filtering by program")]
    Filter { value: Option<String> },
    #[structopt(about = "Enables writing traces each in different file")]
    Multiple { value: Option<bool> },
    #[structopt(about = "Sets output file name")]
    Output { value: Option<String> },
    #[structopt(about = "Shows the active configuration")]
    Show {},
}

/// Constructs an instance of the Application.
pub fn application() -> Application {
    Application::from_args()
}
