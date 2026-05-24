# Vampus — CLI tool for semantic version management

## Overview

Single-crate Rust CLI (edition 2024) that automates SemVer version bumps across configurable file patterns. Not a workspace.

## Quick reference

```bash
cargo build                    # debug build
cargo build --release          # release build
cargo run -- upgrade --patch   # run CLI
cargo run -- --debug upgrade   # enable tracing debug output (top-level flag, any subcommand)
cargo fmt --check              # format check (no custom config)
cargo clippy -- -D warnings    # lint
```

**No tests exist** — `cargo test` runs zero test functions.

**CodeGraph initialized** — prefer `codegraph_*` tools for structural queries over grep.

## Architecture

| Module | Responsibility |
|--------|---------------|
| `src/cli.rs` | Clap derive CLI (subcommands: upgrade, downgrade, preview, show, init) |
| `src/config.rs` | Serde-deserialized `.vampus.yml` config model |
| `src/utils.rs` | SemVer math, regex replacement with capture-group wrapping, file I/O |
| `src/main.rs` | Async tokio entrypoint, command dispatch, two-phase (simulate → apply) logic |

## Key behavior

- Config file is `.vampus.yml` in **cwd** — not `~/.config/` or project root.
- Search patterns use `{{current_version}}` placeholder; the code automatically wraps it with capture groups `(prefix){{current_version}}(suffix)` for `$1` / `$2` replacement.
- Two-phase transaction: simulate all files first (verify old pattern exists + new pattern matches), then write on success.
- `--patch` is the **default** when no version flag is given.
- `RUST_LOG` env var controls tracing level; `--debug` overrides to `debug`.
- `vampus init` refuses to overwrite an existing `.vampus.yml`.

## Style / conventions

- Uses `Rust 2024` edition — `cargo fix --edition` may be needed if upgrading from older edition.
- No `rustfmt.toml` or `clippy.toml` — all defaults.
- Imports: `std::*` first, then external crates, then `crate::*`.
- Enum variants and struct fields use `#[serde(default)]` for backward compat in config.
- Tracing `error!` for failures, `debug!` for diagnostics, `println!` for user-facing output.