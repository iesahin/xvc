# Xvc Configuration Manager

The `xvc-config` crate provides a general solution for maintaining configuration settings across different sources with cascading precedence. It is part of the [Xvc](https://xvc.dev) project, a version control system focused on data and MLOps.

## Overview

This crate provides a flexible configuration system that can load and merge settings from multiple sources:

- Default values embedded in the code
- System-wide configuration
- User-specific global configuration
- Project configuration (tracked by Git)
- Local project configuration (not tracked by Git)
- Environment variables
- Command line options

Configuration values are cascaded and overridden according to precedence, with command line options having the highest priority.

## Features

- **Multiple Configuration Sources**: Load configuration from files, environment variables, and command-line arguments
- **Hierarchical Configuration**: Support for nested configuration groups using dot notation (`group.subgroup.key`)
- **Type-Safe Access**: Get configuration values as specific types (string, bool, int, float)
- **Source Tracking**: Each configuration value is tracked to its source
- **TOML Format**: Configuration files use the TOML format for human readability
- **Extensible**: Add custom configuration sources or parsers

## Usage



Here are examples showing how to use the xvc-config crate in various scenarios:

### Creating Configuration Parameters

```rust
use std::path::PathBuf;
use xvc_config::{XvcConfigParams, XvcConfig};
use xvc_walker::AbsolutePath;

// Create custom configuration parameters
let params = XvcConfigParams {
    // Set the current directory
    current_dir: AbsolutePath::from(std::env::current_dir().unwrap()),
    
    // Set default configuration
    default_configuration: r#"
        [core]
        guid = ""
        verbosity = "info"
        
        [storage]
        type = "local"
        path = "./data"
    "#.to_string(),
    
    // Include standard config locations
    include_system_config: true,
    include_user_config: true,
    include_environment_config: true,
    
    // Specify custom paths
    project_config_path: Some(PathBuf::from("./xvc.toml")),
    local_config_path: Some(PathBuf::from("./xvc.local.toml")),
    
    // Override with command line options
    command_line_config: Some(vec![
        "core.verbosity=debug".to_string(),
        "storage.path=./custom-data".to_string(),
    ]),
};

// Create config from parameters
let config = XvcConfig::new(params).expect("Failed to create config");
