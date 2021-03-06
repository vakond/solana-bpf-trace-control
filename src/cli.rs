//! solana-bpf-trace-control command line interface definition.

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Solana BPF trace control program")]
pub struct Application {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Enables or disables BPF tracing")]
    Enable { value: Option<bool> },

    #[structopt(about = "Applies filtering by program")]
    Filter { value: Option<String> },

    #[structopt(about = "Sets output file name")]
    Output { value: Option<String> },

    #[structopt(about = "Forces writing traces as raw binary")]
    Binary { value: Option<bool> },

    #[structopt(about = "Enables writing traces each in different file")]
    MultipleFiles { value: Option<bool> },

    #[structopt(about = "Limits number of working threads")]
    MaxThreads { value: Option<usize> },

    #[structopt(about = "Filters out too short traces")]
    MinLength { value: Option<usize> },
}

/// Constructs an instance of the Application.
pub fn application() -> Application {
    Application::from_args()
}
