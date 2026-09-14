use bollard::plugin::{ContainerSummary, ImageSummary};

pub fn short_id(id: &str) -> String {
    id.strip_prefix("sha256:")
        .unwrap_or(id)
        .chars()
        .take(12)
        .collect()
}

pub fn format_size(bytes: i64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, units[unit])
    } else {
        format!("{:.2} {}", size, units[unit])
    }
}

pub fn truncate_text(value: &str, max_width: usize) -> String {
    if value.chars().count() <= max_width {
        return value.into();
    }

    format!(
        "{}...",
        value.chars().take(max_width - 3).collect::<String>()
    )
}

pub async fn print_images(images: Vec<ImageSummary>) {
    println!("{:<12} {:<40} {}", "IMAGE ID", "TAGS", "SIZE");

    for image in images {
        let id = short_id(&image.id);

        let tags = if image.repo_tags.is_empty() {
            "-".into()
        } else {
            image.repo_tags.join(", ")
        };

        println!(
            "{:<12} {:<40} {}",
            id,
            truncate_text(&tags, 40),
            format_size(image.size)
        );
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
            truncate_text(&names.join(","), 24),
            truncate_text(&image, 24),
            status
        );
    }
}
