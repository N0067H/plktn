# plktn

A small Rust Docker CLI for async container control.

`plktn` is short for `plankton`. It connects to the local Docker daemon and provides a compact set of container and image commands.

## Features

- List running containers
- List all containers
- List local images
- Stop multiple containers concurrently
- Stream container logs

## Requirements

- Rust
- Docker daemon running locally
- Access to the Docker socket

By default, plktn uses Docker's local defaults, such as:

```text
unix:///var/run/docker.sock
```

## Usage

List running containers:

```bash
cargo run -- ps
```

List all containers:

```bash
cargo run -- ps -a
```

List images:

```bash
cargo run -- images
```

Stop one or more containers:

```bash
cargo run -- stop <container>...
```

Stream logs:

```bash
cargo run -- logs -f <container>
```

Print existing logs and exit:

```bash
cargo run -- logs <container>
```

## Development

Check the project:

```bash
cargo check
```

Format the code:

```bash
cargo fmt
```
