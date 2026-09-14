use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Ps {
        #[arg(short, long)]
        all: bool,
    },
    Images,
    Stop {
        #[arg(short, long, conflicts_with = "containers")]
        all: bool,

        #[arg(required_unless_present = "all")]
        containers: Vec<String>,
    },
    Logs {
        #[arg(short, long)]
        follow: bool,

        #[arg(required = true)]
        container: String,
    },
    Start {
        #[arg(required = true)]
        containers: Vec<String>,
    },
    Restart {
        #[arg(required = true)]
        containers: Vec<String>,
    },
    Rm {
        #[arg(required = true)]
        containers: Vec<String>,
    },
}
