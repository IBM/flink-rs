# flink-rs

[![CI](https://github.com/IBM/flink-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/IBM/flink-rs/actions/workflows/ci.yml)

A Rust CLI for interacting with a running [Apache Flink](https://flink.apache.org/) cluster via the [REST API v1](https://nightlies.apache.org/flink/flink-docs-release-2.3/generated/rest_v1_dispatcher.yml) (Flink 2.3).

> First release — welcome to the beta. Feedback and contributions are very welcome.

## Install

### Build from source

```bash
git clone https://github.com/IBM/flink-rs
cd flink-rs
cargo build --release
# binary at: target/release/flink-rs
```

### Pre-built Linux binary (musl static — no runtime deps)

```bash
# One-time build setup
rustup target add x86_64-unknown-linux-musl
sudo dnf install -y musl-gcc   # Fedora/RHEL
# sudo apt install -y musl-tools  # Debian/Ubuntu

cargo build --release --target x86_64-unknown-linux-musl
# binary at: target/x86_64-unknown-linux-musl/release/flink-rs
```

Or with `just`:

```bash
just dist-linux        # x86-64 musl static
just dist-linux-arm64  # ARM64 musl static (needs gcc-aarch64-linux-gnu)
just dist-all          # both Linux targets
```

## Configuration

Save your cluster URL once — no need to pass `--host` every time:

```bash
flink-rs config set host http://localhost:8081
```

Config is stored at `~/.config/flink-cli/config.toml`.

Override per-command with `--host` or the `FLINK_HOST` environment variable:

```bash
FLINK_HOST=http://prod:8081 flink-rs cluster overview
flink-rs --host http://staging:8081 jobs list
```

## Usage

### Cluster

```bash
# Overview and health status
flink-rs cluster overview

# Full cluster configuration
flink-rs cluster config

# Shut down the cluster (destructive — requires --yes)
flink-rs cluster shutdown --yes
```

Example output:

```
Flink 2.3.0  commit a1b2c3

 Metric          │ Value
─────────────────┼───────
 Jobs Running    │ 2
 Jobs Finished   │ 14
 Jobs Cancelled  │ 0
 Jobs Failed     │ 1
 Task Managers   │ 3
 Slots Total     │ 12
 Slots Available │ 6

Status: HEALTHY ✓
```

### Task Managers

```bash
flink-rs taskmanagers list
```

### Jobs

```bash
# List all jobs
flink-rs jobs list

# Details for a single job
flink-rs jobs get <job-id>

# Cancel a job
flink-rs jobs cancel <job-id>

# Get execution result (finished jobs)
flink-rs jobs result <job-id>

# Watch a job live in the terminal (q or Ctrl-C to exit)
flink-rs jobs watch <job-id>
flink-rs jobs watch <job-id> --interval 5   # refresh every 5s (default: 3)
```

### JARs & Job Submission

```bash
# List uploaded JARs
flink-rs jars list

# Upload a JAR
flink-rs jars upload ./my-job.jar

# Run an already-uploaded JAR (waits for completion by default)
flink-rs jars run <jar-id> \
  --entry-class com.example.Main \
  --parallelism 4 \
  --args "arg1,arg2"

# Upload + run in one step
flink-rs jars submit ./my-job.jar \
  --entry-class com.example.Main \
  --parallelism 4

# Return immediately with the job ID instead of waiting
flink-rs jars submit ./my-job.jar --no-wait

# Delete an uploaded JAR
flink-rs jars delete <jar-id>
```

### Output formats

Every command supports `--output json` for scripting:

```bash
flink-rs --output json jobs list | jq '.jobs[] | select(.state=="RUNNING")'
flink-rs --output json cluster overview | jq '.["slots-available"]'

# Save as default
flink-rs config set output json
```

## Global flags

| Flag                   | Env var      | Description                    |
| ---------------------- | ------------ | ------------------------------ |
| `--host <URL>`         | `FLINK_HOST` | Flink JobManager REST URL      |
| `--output table\|json` | —            | Output format (default: table) |

## Development

```bash
just build    # debug build
just release  # optimised release
just ci       # fmt check + clippy + build + test
just fmt      # auto-format
just lint     # clippy -D warnings
```

## Cross-compile targets

| Task                    | Target              | Needs                    |
| ----------------------- | ------------------- | ------------------------ |
| `just dist-linux`       | x86-64 musl static  | `musl-gcc`               |
| `just dist-linux-arm64` | ARM64 musl static   | `gcc-aarch64-linux-gnu`  |
| `just dist-all`         | both Linux targets  | above                    |
| `just dist-macos-x86`   | macOS x86-64        | `cargo-zigbuild` + `zig` |
| `just dist-macos-arm64` | macOS Apple Silicon | `cargo-zigbuild` + `zig` |
| `just dist-windows`     | Windows x86-64      | `cross` + Docker         |
| `just dist-all-cross`   | all 5 targets       | above                    |

Install `cross`: `cargo install cross --git https://github.com/cross-rs/cross`  
Install `cargo-zigbuild`: `cargo install cargo-zigbuild` (requires `zig` — `sudo dnf install -y zig`)

## API coverage

Based on the [Flink 2.3 REST API spec](https://nightlies.apache.org/flink/flink-docs-release-2.3/generated/rest_v1_dispatcher.yml).

| Endpoint                          | Implemented |
| --------------------------------- | ----------- |
| `GET /overview`                   | ✅          |
| `GET /jobmanager/config`          | ✅          |
| `DELETE /cluster`                 | ✅          |
| `GET /taskmanagers`               | ✅          |
| `GET /jobs/overview`              | ✅          |
| `GET /jobs/{id}`                  | ✅          |
| `DELETE /jobs/{id}`               | ✅ (cancel) |
| `GET /jobs/{id}/execution-result` | ✅          |
| `GET /jobs/{id}/status`           | ✅          |
| `GET /jars`                       | ✅          |
| `POST /jars/upload`               | ✅          |
| `POST /jars/{id}/run`             | ✅          |
| `DELETE /jars/{id}`               | ✅          |

## License

All source files include a Copyright and License header. The SPDX license header is preferred.

```text
#
# Copyright IBM Corp. 2025
# SPDX-License-Identifier: Apache-2.0
#
```

If you would like to see the detailed LICENSE click [here](LICENSE).

If you have any questions or issues you can create a new [issue here][issues].

Pull requests are very welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

[issues]: https://github.com/IBM/flink-rs/issues/new
