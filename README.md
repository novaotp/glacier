# Coolify Backups

A small script to backup the data in some of my services hosted with Coolify.

## Installation

### Prerequisites

- [Rust](https://rust-lang.org/tools/install/) `1.96+`

### Development

1. Create an `.env` file based on `.env.example`.

    > You only need to modify the services section.

    ```sh
    cp .env.example .env
    ```

2. Run the `compose.yaml` for a local S3.

    > The UI is located at https://localhost:9191.

    ```sh
    sudo docker compose up --build
    ```

3. Run the tool.

    ```sh
    cargo run
    ```

### Production

1. Build the release version.

    ```sh
    cargo build --release
    ```

2. Run the executable.

    ```sh
    ./target/release/coolify-backups
    ```
