mod cli;
mod docker;
mod output;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    docker::{get_container_list, get_images, log, stop_container},
    output::{print_containers, print_images},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Ps { all } => {
            let containers = get_container_list(*all).await?;
            print_containers(containers).await;
        }
        Command::Images => {
            let images = get_images().await?;
            print_images(images).await;
        }
        Command::Stop { containers } => stop_container(containers).await?,
        Command::Logs { follow, container } => log(*follow, container).await?,
    }

    Ok(())
}
