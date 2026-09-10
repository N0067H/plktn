use anyhow::Context;
use bollard::{
    Docker,
    query_parameters::{
        ListContainersOptionsBuilder, ListImagesOptionsBuilder, LogsOptionsBuilder,
        StopContainerOptionsBuilder,
    },
};
use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use futures_util::future::join_all;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    Ps {
        #[arg(short, long)]
        all: bool,
    },
    Images,
    Stop {
        #[arg(required = true)]
        containers: Vec<String>,
    },
    Logs {
        #[arg(short, long)]
        follow: bool,

        #[arg(required = true)]
        container: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Ps { all } => {
            let docker = connect_docker().await?;
            let options = ListContainersOptionsBuilder::default().all(*all).build();
            let containers = docker.list_containers(Some(options)).await?;

            println!(
                "{:<12} {:<24} {:<24} {}",
                "CONTAINER ID", "NAME", "IMAGE", "STATUS"
            );

            for container in containers {
                let id = container
                    .id
                    .as_deref()
                    .map(short_id)
                    .unwrap_or_else(|| "-".into());

                let names: Vec<String> = container
                    .names
                    .unwrap_or_default()
                    .into_iter()
                    .map(|name| name.strip_prefix('/').unwrap_or(&name).into())
                    .collect();

                let image = container.image.unwrap_or_else(|| "-".into());

                let status = container.status.unwrap_or_else(|| "-".into());

                println!(
                    "{:<12} {:<24} {:<24} {}",
                    id,
                    names.join(","),
                    image,
                    status
                );
            }
        }
        Command::Images => {
            let docker = connect_docker().await?;
            let options = ListImagesOptionsBuilder::default().build();
            let images = docker.list_images(Some(options)).await?;

            println!("{:<12} {:<40} {}", "IMAGE ID", "TAGS", "SIZE");

            for image in images {
                let id = short_id(&image.id);

                let tags = if image.repo_tags.is_empty() {
                    "-".into()
                } else {
                    image.repo_tags.join(",")
                };

                println!("{:<12} {:<40} {}", id, tags, image.size);
            }
        }
        Command::Stop { containers } => {
            let docker = connect_docker().await?;
            let options = StopContainerOptionsBuilder::default().build();

            let tasks = containers.iter().map(|container| {
                let docker = docker.clone();
                let options = options.clone();
                let container = container.clone();

                async move {
                    let result = docker.stop_container(&container, Some(options)).await;
                    (container, result)
                }
            });

            let results = join_all(tasks).await;

            for (container, result) in results {
                match result {
                    Ok(_) => println!("{container}: stopped"),
                    Err(err) => println!("{container}: failed: {err}"),
                }
            }
        }
        Command::Logs { follow, container } => {
            let docker = connect_docker().await?;
            let options = LogsOptionsBuilder::default()
                .stdout(true)
                .stderr(true)
                .follow(*follow)
                .tail("all")
                .build();
            let mut logs = docker.logs(container, Some(options));

            while let Some(log) = logs.next().await {
                match log {
                    Ok(output) => print!("{output}"),
                    Err(err) => eprintln!("{container} failed to read logs: {err}"),
                }
            }
        }
    }

    Ok(())
}

async fn connect_docker() -> anyhow::Result<Docker> {
    let docker = Docker::connect_with_local_defaults().context("failed to create Docker client")?;
    docker.ping().await.context("failed to connect to Docker daemon; check whether Docker is running and the socket is accessible")?;
    Ok(docker)
}

fn short_id(id: &str) -> String {
    id.strip_prefix("sha256:")
        .unwrap_or(id)
        .chars()
        .take(12)
        .collect()
}
