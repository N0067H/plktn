use anyhow::Context;
use bollard::{
    Docker,
    plugin::{ContainerSummary, ImageSummary},
    query_parameters::{
        ListContainersOptionsBuilder, ListImagesOptionsBuilder, LogsOptionsBuilder,
        StopContainerOptionsBuilder,
    },
};
use futures_util::{StreamExt, future::join_all};

pub async fn connect_docker() -> anyhow::Result<Docker> {
    let docker = Docker::connect_with_local_defaults().context("failed to create Docker client")?;
    docker.ping().await.context("failed to connect to Docker daemon; check whether Docker is running and the socket is accessible")?;
    anyhow::Ok(docker)
}

pub async fn get_container_list(all: bool) -> anyhow::Result<Vec<ContainerSummary>> {
    let docker = connect_docker().await?;
    let options = ListContainersOptionsBuilder::default().all(all).build();

    let containers = docker.list_containers(Some(options)).await?;
    anyhow::Ok(containers)
}

pub async fn get_images() -> anyhow::Result<Vec<ImageSummary>> {
    let docker = connect_docker().await?;
    let options = ListImagesOptionsBuilder::default().build();

    let images = docker.list_images(Some(options)).await?;
    anyhow::Ok(images)
}

pub async fn stop_container(containers: &[String]) -> anyhow::Result<()> {
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

    anyhow::Ok(())
}

pub async fn log(follow: bool, container: &str) -> anyhow::Result<()> {
    let docker = connect_docker().await?;
    let options = LogsOptionsBuilder::default()
        .stdout(true)
        .stderr(true)
        .follow(follow)
        .tail("all")
        .build();
    let mut logs = docker.logs(container, Some(options));

    while let Some(log) = logs.next().await {
        match log {
            Ok(output) => print!("{output}"),
            Err(err) => eprintln!("{container} failed to read logs: {err}"),
        }
    }

    anyhow::Ok(())
}
