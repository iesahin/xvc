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



### Basic Usage
