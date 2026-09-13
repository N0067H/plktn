use bollard::plugin::{ContainerSummary, ImageSummary};

pub fn short_id(id: &str) -> String {
    id.strip_prefix("sha256:")
        .unwrap_or(id)
        .chars()
        .take(12)
        .collect()
}

pub async fn print_images(images: Vec<ImageSummary>) {
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

pub async fn print_containers(containers: Vec<ContainerSummary>) {
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
