# Coolify Backups

A small script to backup the data in some of my services hosted with Coolify.

## Installation

### Prerequisites

- [Rust](https://rust-lang.org/tools/install/) `1.96+`

### Development

1. Run the tool.

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
