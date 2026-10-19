# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.1.0] - 2025-01-01

### Added
- Initial release of `flink-rs` — a Rust CLI for Apache Flink clusters
- `cluster overview` — health summary (jobs, task managers, slots)
- `cluster config` — full JobManager configuration dump
- `cluster shutdown --yes` — graceful cluster shutdown
- `taskmanagers list` — list all task managers
- `jobs list` — list all jobs with state and duration
- `jobs get <id>` — detailed job information
- `jobs cancel <id>` — cancel a running job
- `jobs result <id>` — fetch execution result for a finished job
- `jobs watch <id>` — live in-place TUI job monitor (ratatui)
- `jars list` — list uploaded JARs
- `jars upload <path>` — upload a JAR file
- `jars run <id>` — run a previously uploaded JAR
- `jars submit <path>` — upload and run in one step
- `jars delete <id>` — delete an uploaded JAR
- `--output json` flag on every command for scripting
- `config set / show / path` for persistent host and output configuration
- Pre-built archives for Linux x86-64 (musl), Linux ARM64, macOS x86-64, macOS ARM64, and Windows x86-64

[unreleased]: https://github.com/IBM/flink-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/IBM/flink-rs/releases/tag/v0.1.0
