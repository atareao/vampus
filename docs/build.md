# Building Vampus

## Prerequisites

- Rust 1.89.0 or newer (MSRV for `noyalib` dependency)
- Linux, macOS, or Windows

## Debug build

```bash
cargo build
```

## Release build

```bash
cargo build --release
```

## Static binary (musl)

For a fully static binary with no glibc dependency:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

The resulting binary works on any Linux distribution including Alpine and NixOS.

## CI/CD

GitHub Actions workflows are in `.github/workflows/`:

| Workflow | Trigger | Purpose |
|---|---|---|
| `ci.yml` | PRs to main/development | Format, clippy, build, test |
| `release-prepare.yml` | Push to main | Bump version, changelog, tag |
| `release.yml` | Tag v* | Static binary builds, crates.io publish |

## Dependencies

Vampus keeps its dependency tree minimal. After the migration from `serde_yaml` to `noyalib` (v0.4.7), all dependencies are pure Rust with no unsafe code in YAML parsing.

Run `cargo audit` to check for advisories before committing.
