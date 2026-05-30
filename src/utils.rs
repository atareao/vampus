use std::{env, io, path::PathBuf};

use regex::Regex;
use tokio::fs;
use tracing::debug;

use crate::cli::VersionArgs;

/// Errors that can occur during version calculation.
#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("Invalid version format: {0}")]
    InvalidFormat(String),
    #[error("{0}")]
    NegativeVersion(String),
    #[error("Unknown change type: {0}")]
    UnknownChangeType(String),
}

const APP_NAME: &str = "vampus";

/// Wraps the user's search pattern with capture groups around
/// the text before and after the `{{current_version}}` marker.
///
/// Example: `"^version = \"{{current_version}}\"$"` becomes
/// `"(^version = \"){{current_version}}(\"$)"`.
pub fn wrap_search_pattern(search_pattern: &str) -> String {
    let parts: Vec<&str> = search_pattern.split("{{current_version}}").collect();

    if parts.len() < 2 {
        return search_pattern.to_string();
    }

    let prefix = parts[0];
    let suffix = parts[1];

    format!("({prefix}){{{{current_version}}}}({suffix})")
}

/// Simulates a regex replacement on a file, then verifies the new pattern exists.
pub async fn simulate_replacement(
    path: &str,
    pattern_from: &str,
    replacement_to: &str,
    pattern_to: &str,
) -> Result<String, io::Error> {
    let re_from = Regex::new(pattern_from).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error compiling RegEx FROM '{}': {}", pattern_from, e),
        )
    })?;

    let re_to = Regex::new(pattern_to).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error compiling RegEx TO '{}': {}", pattern_to, e),
        )
    })?;

    let content_bytes = fs::read(path).await?;
    let content = String::from_utf8(content_bytes)
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("File '{}' does NOT contain valid UTF-8 text: {}", path, e),
            )
        })?
        .replace('\r', "");

    if !re_from.is_match(&content) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Last pattern '{}' NOT found in '{}'.", pattern_from, path),
        ));
    }

    let modified_content = re_from.replace_all(&content, replacement_to);
    debug!("Content modified simulated:\n{}", modified_content);

    if re_to.is_match(&modified_content) {
        Ok(modified_content.into_owned())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "New version pattern '{}' not found after simulation. Replacement did not match the expected format.",
                pattern_to
            ),
        ))
    }
}

/// Writes pre-calculated content to a file, replacing existing content.
pub async fn apply_replacement(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content.as_bytes()).await
}

pub enum Operation {
    Increment,
    Decrement,
}

/// Returns the SemVer change type from CLI flags.
///
/// Defaults to `"patch"` when no flag is given.
pub fn get_version_change(args: &VersionArgs) -> &'static str {
    if args.major {
        "major"
    } else if args.minor {
        "minor"
    } else {
        "patch"
    }
}

/// Computes the new version string by incrementing or decrementing the
/// specified SemVer component of `current_version`.
pub fn calculate_version(
    current_version: &str,
    change_type: &str,
    operation: Operation,
) -> Result<String, VersionError> {
    let parts: Vec<&str> = current_version.split('.').collect();
    if parts.len() != 3 {
        return Err(VersionError::InvalidFormat(current_version.to_string()));
    }

    let mut major = parts[0]
        .parse::<i32>()
        .map_err(|_| VersionError::InvalidFormat(current_version.to_string()))?;
    let mut minor = parts[1]
        .parse::<i32>()
        .map_err(|_| VersionError::InvalidFormat(current_version.to_string()))?;
    let mut patch = parts[2]
        .parse::<i32>()
        .map_err(|_| VersionError::InvalidFormat(current_version.to_string()))?;

    match operation {
        Operation::Increment => match change_type {
            "major" => {
                major += 1;
                minor = 0;
                patch = 0;
            }
            "minor" => {
                minor += 1;
                patch = 0;
            }
            "patch" => {
                patch += 1;
            }
            _ => return Err(VersionError::UnknownChangeType(change_type.to_string())),
        },
        Operation::Decrement => match change_type {
            "major" => {
                if major == 0 {
                    return Err(VersionError::NegativeVersion(
                        "Cannot downgrade major version 0".to_string(),
                    ));
                }
                major -= 1;
                minor = 0;
                patch = 0;
            }
            "minor" => {
                if minor == 0 && major == 0 {
                    return Err(VersionError::NegativeVersion(
                        "Cannot downgrade minor 0 when major is 0".to_string(),
                    ));
                } else if minor == 0 {
                    return Err(VersionError::NegativeVersion(
                        "Cannot downgrade minor 0 without explicitly specifying --major"
                            .to_string(),
                    ));
                }
                minor -= 1;
                patch = 0;
            }
            "patch" => {
                if patch == 0 && minor == 0 && major == 0 {
                    return Err(VersionError::NegativeVersion(
                        "Cannot downgrade 0.0.0".to_string(),
                    ));
                } else if patch == 0 {
                    return Err(VersionError::NegativeVersion(
                        "Cannot downgrade patch 0 without explicitly specifying --minor or --major"
                            .to_string(),
                    ));
                }
                patch -= 1;
            }
            _ => return Err(VersionError::UnknownChangeType(change_type.to_string())),
        },
    }

    if major < 0 || minor < 0 || patch < 0 {
        return Err(VersionError::NegativeVersion(
            "Invalid (negative) version result".to_string(),
        ));
    }

    Ok(format!("{}.{}.{}", major, minor, patch))
}

/// Returns the path to the `.vampus.yml` config file in the current directory.
pub async fn get_config_path() -> PathBuf {
    let mut config_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    config_path.push(format!(".{}.yml", APP_NAME));
    config_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::VersionArgs;

    mod calculate_version {
        use super::*;

        mod increment {
            use super::*;

            #[test]
            fn increments_patch_for_1_2_3() {
                let result = calculate_version("1.2.3", "patch", Operation::Increment);
                assert_eq!(result.unwrap(), "1.2.4");
            }

            #[test]
            fn increments_minor_for_1_2_3() {
                let result = calculate_version("1.2.3", "minor", Operation::Increment);
                assert_eq!(result.unwrap(), "1.3.0");
            }

            #[test]
            fn increments_major_for_1_2_3() {
                let result = calculate_version("1.2.3", "major", Operation::Increment);
                assert_eq!(result.unwrap(), "2.0.0");
            }

            #[test]
            fn wraps_minor_on_major_increment() {
                let result = calculate_version("0.5.9", "major", Operation::Increment);
                assert_eq!(result.unwrap(), "1.0.0");
            }

            #[test]
            fn wraps_patch_on_minor_increment() {
                let result = calculate_version("1.2.9", "minor", Operation::Increment);
                assert_eq!(result.unwrap(), "1.3.0");
            }

            #[test]
            fn errors_on_unknown_change_type() {
                let result = calculate_version("1.0.0", "invalid", Operation::Increment);
                assert!(result.is_err());
            }
        }

        mod decrement {
            use super::*;

            #[test]
            fn decrements_patch_for_1_2_3() {
                let result = calculate_version("1.2.3", "patch", Operation::Decrement);
                assert_eq!(result.unwrap(), "1.2.2");
            }

            #[test]
            fn decrements_minor_for_1_2_3() {
                let result = calculate_version("1.2.3", "minor", Operation::Decrement);
                assert_eq!(result.unwrap(), "1.1.0");
            }

            #[test]
            fn decrements_major_for_2_0_0() {
                let result = calculate_version("2.0.0", "major", Operation::Decrement);
                assert_eq!(result.unwrap(), "1.0.0");
            }

            #[test]
            fn errors_on_patch_zero_without_minor() {
                let result = calculate_version("1.0.0", "patch", Operation::Decrement);
                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Cannot downgrade patch 0")
                );
            }

            #[test]
            fn errors_on_minor_zero_without_major() {
                let result = calculate_version("1.0.0", "minor", Operation::Decrement);
                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Cannot downgrade minor 0 without")
                );
            }

            #[test]
            fn errors_on_minor_zero_when_major_is_zero() {
                let result = calculate_version("0.0.0", "minor", Operation::Decrement);
                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Cannot downgrade minor 0 when major is 0")
                );
            }

            #[test]
            fn errors_on_major_zero() {
                let result = calculate_version("0.0.0", "major", Operation::Decrement);
                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Cannot downgrade major version 0")
                );
            }

            #[test]
            fn errors_on_0_0_0_patch() {
                let result = calculate_version("0.0.0", "patch", Operation::Decrement);
                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Cannot downgrade 0.0.0")
                );
            }

            #[test]
            fn errors_on_unknown_change_type() {
                let result = calculate_version("1.0.0", "invalid", Operation::Decrement);
                assert!(result.is_err());
            }
        }

        mod invalid_input {
            use super::*;

            #[test]
            fn errors_on_missing_dot() {
                let result = calculate_version("1.2", "patch", Operation::Increment);
                assert!(result.is_err());
            }

            #[test]
            fn errors_on_non_numeric() {
                let result = calculate_version("a.b.c", "patch", Operation::Increment);
                assert!(result.is_err());
            }

            #[test]
            fn errors_on_empty_string() {
                let result = calculate_version("", "patch", Operation::Increment);
                assert!(result.is_err());
            }
        }
    }

    mod wrap_search_pattern {
        use super::*;

        #[test]
        fn wraps_with_capture_groups() {
            let result = wrap_search_pattern(r#"^version = "{{current_version}}"$"#);
            assert_eq!(result, r#"(^version = "){{current_version}}("$)"#);
        }

        #[test]
        fn returns_original_when_no_placeholder() {
            let result = wrap_search_pattern(r#"^version = "1.0.0"$"#);
            assert_eq!(result, r#"^version = "1.0.0"$"#);
        }

        #[test]
        fn handles_empty_prefix() {
            let result = wrap_search_pattern("{{current_version}}-beta");
            assert_eq!(result, r#"(){{current_version}}(-beta)"#);
        }

        #[test]
        fn handles_empty_suffix() {
            let result = wrap_search_pattern("v{{current_version}}");
            assert_eq!(result, r#"(v){{current_version}}()"#);
        }
    }

    mod get_version_change {
        use super::*;

        #[test]
        fn returns_major_when_major_flag() {
            let args = VersionArgs {
                major: true,
                minor: false,
                patch: false,
            };
            assert_eq!(get_version_change(&args), "major");
        }

        #[test]
        fn returns_minor_when_minor_flag() {
            let args = VersionArgs {
                major: false,
                minor: true,
                patch: false,
            };
            assert_eq!(get_version_change(&args), "minor");
        }

        #[test]
        fn defaults_to_patch() {
            let args = VersionArgs {
                major: false,
                minor: false,
                patch: false,
            };
            assert_eq!(get_version_change(&args), "patch");
        }

        #[test]
        fn major_takes_precedence_over_minor() {
            let args = VersionArgs {
                major: true,
                minor: true,
                patch: false,
            };
            assert_eq!(get_version_change(&args), "major");
        }
    }
}
