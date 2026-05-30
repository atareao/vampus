# Shell Completions Implementation Plan

## Overview
Add a `completions` subcommand to the vampus CLI that generates shell completion scripts using `clap_complete`.

## Changes

### 1. Cargo.toml
- Add dependency: `clap_complete = "4.5"`

### 2. src/cli.rs
- Add `ValueEnum` to clap imports: `use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};`
- Add new subcommand variant:
  ```rust
  /// Generate shell completion scripts.
  Completions(ShellArgs),
  ```
- Add new args struct:
  ```rust
  #[derive(Args)]
  pub struct ShellArgs {
      /// The shell to generate completions for.
      #[arg(value_enum)]
      pub shell: clap_complete::Shell,
  }
  ```

### 3. src/main.rs
- Import `clap::CommandFactory` and `clap_complete`
- Add match arm in the command dispatch:
  ```rust
  Commands::Completions(args) => {
      let mut cmd = Cli::command();
      let name = cmd.get_name().to_string();
      clap_complete::generate(args.shell, &mut cmd, name, &mut std::io::stdout());
  }
  ```

## Git Workflow
1. Squash merge `feat/shell-completions` → `development`
2. Create PR `development` → `main`

## Verification
- `cargo build` compiles successfully
- `cargo clippy -- -D warnings` passes
- `vampus completions bash` outputs bash completions
- `vampus completions fish` outputs fish completions