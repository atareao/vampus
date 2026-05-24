use std::{env, str::FromStr};

use clap::Parser;
use tracing::{debug, error};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod config;
mod utils;
use cli::{Cli, Commands, VersionArgs};
use config::Config;
use utils::{
    Operation, apply_replacement, calculate_version, get_config_path, get_version_change,
    simulate_replacement, wrap_search_pattern,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let log_filter_str = if cli.debug {
        "debug".to_string()
    } else {
        env::var("RUST_LOG").unwrap_or("ERROR".to_string())
    };

    tracing_subscriber::registry()
        .with(
            EnvFilter::from_str(&log_filter_str)
                .unwrap_or_else(|_| EnvFilter::from_str("error").unwrap()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!("log_level: {}", log_filter_str);

    match &cli.command {
        Commands::Upgrade(args) => {
            execute_version_change(args, Operation::Increment).await;
        }
        Commands::Downgrade(args) => {
            execute_version_change(args, Operation::Decrement).await;
        }
        Commands::Preview(args) => {
            let change_type = get_version_change(args);
            let config_path = get_config_path().await;

            match Config::read(&config_path).await {
                Some(config) => {
                    match calculate_version(
                        &config.current_version,
                        change_type,
                        Operation::Increment,
                    ) {
                        Ok(new_version) => {
                            println!("Current version: {}", config.current_version);
                            println!("Preview version (Increment): {}", new_version);
                        }
                        Err(e) => {
                            error!("Error calculating the version: {}", e);
                        }
                    }
                }
                None => error!("Failed to read config file at {}", config_path.display()),
            }
        }
        Commands::Show => {
            let config_path = get_config_path().await;
            match Config::read(&config_path).await {
                Some(config) => {
                    println!("{}", config.current_version);
                }
                None => error!("Failed to read config file at {}", config_path.display()),
            }
        }
        Commands::Init => {
            let config_path = get_config_path().await;

            if config_path.exists() {
                error!(
                    "Configuration file already exists at {}",
                    config_path.display()
                );
                return;
            }

            if let Err(e) = Config::write_default(&config_path).await {
                error!("Failed to write default config: {}", e);
                return;
            }
            println!(
                "✅ Created default configuration file: {}",
                config_path.display()
            );
        }
    }
}

async fn execute_version_change(args: &VersionArgs, operation: Operation) {
    let change_type = get_version_change(args);
    let config_path = get_config_path().await;

    let Some(mut config) = Config::read(&config_path).await else {
        error!("Failed to read config file at {}", config_path.display());
        return;
    };

    let new_version = match calculate_version(&config.current_version, change_type, operation) {
        Ok(v) => v,
        Err(e) => {
            error!("Error calculating the version: {}", e);
            return;
        }
    };

    println!("Current version: {}", config.current_version);
    println!("New version: {}", new_version);

    let replacement_to = format!("${{1}}{}${{2}}", new_version);

    let mut modified_files = Vec::new();
    let mut all_files_verified = true;

    println!("-- Verifying and simulating changes... --");

    for replace in &config.replaces {
        let wrapped_search = wrap_search_pattern(replace.pattern.as_str());
        debug!("Wrapped search pattern: {}", wrapped_search);

        let pattern_from = format!(
            "(?m){}",
            wrapped_search.replace(
                "{{current_version}}",
                &config.current_version.replace('.', "\\.")
            )
        );
        debug!("Pattern FROM: {}", pattern_from);

        let pattern_to = format!(
            "(?m){}",
            wrapped_search.replace("{{current_version}}", &new_version.replace('.', "\\."))
        );
        debug!("Pattern TO: {}", pattern_to);

        match simulate_replacement(
            replace.file.as_str(),
            &pattern_from,
            &replacement_to,
            &pattern_to,
        )
        .await
        {
            Ok(content) => {
                modified_files.push((replace.file.clone(), content));
            }
            Err(e) => {
                error!(File = %replace.file, "Simulation failure: {}", e);
                all_files_verified = false;
                break;
            }
        }
    }

    if all_files_verified {
        println!("-- Applying changes... --");

        for (file_path, content) in modified_files {
            match apply_replacement(file_path.as_str(), &content).await {
                Ok(_) => {
                    println!("✅ Updated: {}", file_path);
                }
                Err(e) => {
                    error!(File = %file_path, "Write failure: {}", e);
                }
            }
        }

        config.current_version = new_version;
        if let Err(e) = config.write(&config_path).await {
            error!("Failed to write config file: {}", e);
        } else {
            println!(
                "\n🎉 Success: Config version updated to {}",
                config.current_version
            );
        }
    } else {
        error!("Aborted. No changes were written to files.");
    }
}
