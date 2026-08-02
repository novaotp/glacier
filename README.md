# Glacier

Glacier is a backup service for preserving data from the software I use every day.

## Features

- Backup one or many services.
- Upload backups in one or many storage targets
- File encryption/decryption

## Quick Start

1. Install the binary.

2. Set up the configuration.

    ```sh
    cp .config/glacier.toml.example .config/glacier.toml
    ```

3. Backup of all services once.

    ```sh
    glacier backup --all-services
    ```

### Service Requirements

| Service   | Comment                                                                                                                                                              |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bitwarden | The [Bitwarden CLI](https://bitwarden.com/help/cli/#download-and-install) (`bw`) must be installed and available in `PATH`.                                          |
| Ente      | The [Ente CLI](https://github.com/ente/ente/releases) (`ente`) must be installed and available in `PATH`. An account of type `auth` must also be created before use. |
| Nextcloud | None.                                                                                                                                                                |
| Outline   | None.                                                                                                                                                                |

### Configuration

Glacier supports configuration through :

> Each configuration overrides the previous one in order.

1. Global config file: `~/.config/glacier.toml`
2. Local config file: `.config/glacier.toml`
3. Environment variables (including `.env` files)

## Installation

### Prerequisites

- [Rust](https://rust-lang.org/tools/install/) `1.96+`

### Development

1. Create an `.env` file based on `.env.example`.

    ```sh
    cp .env.example .env
    ```

2. (Optional) Run the `compose.yaml` for a local S3.

    > The UI is located at https://localhost:9191.

    ```sh
    sudo docker compose up --build
    ```

3. Run the tool.

    ```sh
    cargo run -- --help
    ```

### Production

1. Build the release version.

    ```sh
    cargo build --release
    ```

2. Run the executable.

    ```sh
    ./target/release/glacier
    ```

## License

Distributed under the MIT license. See [LICENSE](./LICENSE) for more information.
