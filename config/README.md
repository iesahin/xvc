# Xvc Configuration Manager

The `xvc-config` crate provides a general solution for maintaining configuration settings across different sources with cascading precedence. It is part of the [Xvc](https://xvc.dev) project, a version control system focused on data and MLOps.

## Core Concepts

This crate provides a flexible configuration system that loads and merges settings from multiple sources in a defined order of precedence. Later sources override the settings from earlier ones. The standard hierarchy is:

1.  **Default Values**: Hard-coded defaults that provide a baseline.
2.  **System-wide Configuration**: For system-level settings (e.g., `/etc/xvc/config.toml`).
3.  **User-specific Configuration**: For user-level settings (e.g., `~/.config/xvc/config.toml`).
4.  **Project Configuration**: Stored in the repository, shared with all contributors (e.g., `.xvc/config.toml`).
5.  **Local Project Configuration**: Specific to a local clone of the repository, ignored by Git (e.g., `.xvc/config.local.toml`).
6.  **Environment Variables**: Allows overriding configuration with environment variables (e.g., `XVC_CORE_VERBOSITY=info`).
7.  **Command-line Arguments**: The highest precedence, for on-the-fly modifications.

## Architecture

The configuration system is built around three main structs:

-   **`XvcConfiguration`**: A struct that holds the complete, final configuration for the application. Every field is required and fully populated.
-   **`XvcOptionalConfiguration`**: A parallel struct where every field is wrapped in an `Option`. This is used to represent partial configurations loaded from a single source (like a TOML file or environment variables).
-   **`XvcConfigParams`**: A builder-style struct that specifies *how* to load the configuration. It defines the paths to config files and which sources (system, user, environment, etc.) to include.

The final `XvcConfiguration` is built by:
1.  Starting with a base configuration from `default_config()`.
2.  Loading partial configurations (`XvcOptionalConfiguration`) from each source.
3.  Merging each partial configuration into the base configuration in order of precedence using the `merge_configs` function.

## Usage

The `xvc-config` crate provides the building blocks for configuration management. A higher-level orchestrator would use these components to load and merge the final settings.

### Example: Building a Configuration Manually

This example demonstrates how to use the core components to build a configuration.

```rust
use std::collections::HashMap;
use xvc_config::{
    blank_optional_config, default_config, merge_configs, XvcOptionalConfiguration,
    OptionalCoreConfig,
};

// 1. Start with the default configuration.
let mut config = default_config();
println!("Default verbosity: {}", config.core.verbosity);

// 2. Create a partial configuration, as if loaded from a file or env vars.
let mut user_override = blank_optional_config();
user_override.core = Some(OptionalCoreConfig {
    verbosity: Some("debug".to_string()),
    ..Default::default()
});

// 3. Merge the user override into the base configuration.
config = merge_configs(&config, &user_override);
println!("Verbosity after merge: {}", config.core.verbosity);

// 4. Load configuration from environment variables (or any HashMap).
let mut env_vars = HashMap::new();
env_vars.insert("XVC_CORE_VERBOSITY".to_string(), "warn".to_string());
let env_config = XvcOptionalConfiguration::from_hash_map("XVC_", &env_vars);

// 5. Merge the environment configuration.
config = merge_configs(&config, &env_config);
println!("Final verbosity from env: {}", config.core.verbosity);

assert_eq!(config.core.verbosity, "warn");
```