## [1.1.0] - 2026-07-30

Glacier 1.1.0 adds an improved configuration system.

Previously, the configuration was read from an `.env`. This env file had to be in the current working directory, so you couldn't call the CLI from anywhere. Now, you can configure in 3 ways, with each one overriding the previous one in order :

1. Global config file: `~/.config/glacier.toml`
2. Local config file: `.config/glacier.toml`
3. Environment variables (including `.env` files) _<- same as before_

### Features

- feat(cli): added configuration through toml files ([7eedc05](https://git.lab.itsnova.sh/nova/glacier/commit/7eedc050387ac7cab70da6629ba42eb9a923b240))

## [1.0.0] - 2026-07-28

This marks the first release of Glacier, a backup service for preserving data from the software I use every day.

Glacier 1.0.0 provides support for backing up Bitwarden, Nextcloud, and Outline, with backups stored locally or in S3. It includes a command-line interface, support for services that export multiple backup artifacts, and optional encryption and decryption of backup files.

### Features

- feat: added outline export ([dd75108](https://git.lab.itsnova.sh/nova/glacier/commit/dd7510866c6a316a5fae1e9e0f060881c44f5445))
- feat(local): added local storage ([23eebdb](https://git.lab.itsnova.sh/nova/glacier/commit/23eebdbeae5124f31149e06c52065f1f782d1081))
- feat: added file_extension to ArchiveDescriptor for more flexibility ([5fba4cb](https://git.lab.itsnova.sh/nova/glacier/commit/5fba4cbc4bd62090f319e20c52789aff05817511))
- feat(nextcloud): added nextcloud service ([98ad325](https://git.lab.itsnova.sh/nova/glacier/commit/98ad3255d6c22aebb4c924e240e5253d5939b850))
- feat: added cli ([7564297](https://git.lab.itsnova.sh/nova/glacier/commit/75642975c338fd11728adf210fbecfe6c98c61a4))
- feat(service): added support for multiple export items ([e48867e](https://git.lab.itsnova.sh/nova/glacier/commit/e48867eb8ab61765e2df457ca571001802d99620))
- feat(service): added bitwarden ([5b7fca2](https://git.lab.itsnova.sh/nova/glacier/commit/5b7fca2e3d16ad92e6d96ce6bd082e94a694027a))
- feat(glacier-crypto): added file encryption/decryption package ([2b5b770](https://git.lab.itsnova.sh/nova/glacier/commit/2b5b770389f229346bd61de417ab36243344d7da))
- feat(cli): added file encryption/decryption integration ([c2da041](https://git.lab.itsnova.sh/nova/glacier/commit/c2da041c1e364dde21408385d7ad3eca1c2e39da))
- feat(bitwarden): added glacier-crypto encryption ([518bc95](https://git.lab.itsnova.sh/nova/glacier/commit/518bc951a437d579b7b71d454d02e680474f7b00))

### Bug Fixes

- fix(Cargo.toml): removed publish property ([f97ed21](https://git.lab.itsnova.sh/nova/glacier/commit/f97ed211bec80e68bf043bdef8f14bff52e2e046))

### Maintenance

- refactor: removed environment in favor of config ([f1be0aa](https://git.lab.itsnova.sh/nova/glacier/commit/f1be0aab4bb8364d8522a7fb600d8ce4bea0c2c6))
- refactor(s3): based on a common Storage trait ([16a1f29](https://git.lab.itsnova.sh/nova/glacier/commit/16a1f29b50f8165a07ef5e709c239bea13179546))
- refactor(service): renamed Exporter to Service ([2132c34](https://git.lab.itsnova.sh/nova/glacier/commit/2132c348f4823e19426cb4812c411dc62667e718))
- refactor(service): returning temp files to ease memory usage ([cc8147b](https://git.lab.itsnova.sh/nova/glacier/commit/cc8147bdf74e99bc27516099b4195ffb9a5eaa8d))
- refactor: moved to workspaces ([0a0004f](https://git.lab.itsnova.sh/nova/glacier/commit/0a0004f1785b728515fbf51a1aa1344a6b5db16b))
