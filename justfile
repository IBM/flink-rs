# flink-rs justfile

binary := "flink-rs"

# Show available tasks
default:
    @just --list

# ── dev ──────────────────────────────────────────────────────────────────────

# Dev build (fast, current host)
build:
    cargo build

# Optimised release for current host
release:
    cargo build --release

# Run with arguments, e.g.: just run -- cluster overview
run *args:
    cargo run -- {{args}}

# Run all checks (format, lint, build, test)
ci:
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo build
    cargo test

# Format source
fmt:
    cargo fmt

# Lint
lint:
    cargo clippy -- -D warnings

# ── native cross-compile (no Docker needed) ───────────────────────────────────
# Prereqs (one-time):
#   rustup target add x86_64-unknown-linux-musl
#   rustup target add aarch64-unknown-linux-musl
#   sudo dnf install -y musl-gcc          # x86-64 musl linker
#   sudo dnf install -y gcc-aarch64-linux-gnu  # ARM64 cross linker

# Linux x86-64 static musl binary
dist-linux:
    cargo build --release --target x86_64-unknown-linux-musl
    @echo "Binary: target/x86_64-unknown-linux-musl/release/{{binary}}"

# Linux ARM64 binary (glibc — avoids musl cross-toolchain complexity)
# Prereq: sudo dnf install -y gcc-aarch64-linux-gnu sysroot-aarch64-fc44-glibc
# Linker + sysroot configured in .cargo/config.toml
dist-linux-arm64:
    cargo build --release --target aarch64-unknown-linux-gnu
    @echo "Binary: target/aarch64-unknown-linux-gnu/release/{{binary}}"

# Build all native Linux targets (no Docker needed)
dist-all: dist-linux dist-linux-arm64

# ── cross-compile via zigbuild (macOS) and cross/Docker (Windows) ────────────
# Prereqs:
#   cargo install cross --git https://github.com/cross-rs/cross
#   cargo install cargo-zigbuild
#   sudo dnf install -y zig
#   rustup target add x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-gnu

# macOS x86-64
dist-macos-x86:
    cargo zigbuild --release --target x86_64-apple-darwin
    @echo "Binary: target/x86_64-apple-darwin/release/{{binary}}"

# macOS ARM64 (Apple Silicon)
dist-macos-arm64:
    cargo zigbuild --release --target aarch64-apple-darwin
    @echo "Binary: target/aarch64-apple-darwin/release/{{binary}}"

# Windows x86-64
dist-windows:
    cross build --release --target x86_64-pc-windows-gnu
    @echo "Binary: target/x86_64-pc-windows-gnu/release/{{binary}}.exe"

# Build all targets including macOS + Windows
dist-all-cross: dist-linux dist-linux-arm64 dist-macos-x86 dist-macos-arm64 dist-windows


# ── release archives ──────────────────────────────────────────────────────────

# Package Linux x86-64 musl binary into a .tar.gz
package-linux: dist-linux
    mkdir -p dist
    cp target/x86_64-unknown-linux-musl/release/{{binary}} dist/{{binary}}-linux-x86_64
    tar -czf dist/{{binary}}-linux-x86_64.tar.gz -C dist {{binary}}-linux-x86_64
    @echo "Packaged: dist/{{binary}}-linux-x86_64.tar.gz"

# Package Linux ARM64 binary into a .tar.gz
package-linux-arm64: dist-linux-arm64
    mkdir -p dist
    cp target/aarch64-unknown-linux-gnu/release/{{binary}} dist/{{binary}}-linux-aarch64
    tar -czf dist/{{binary}}-linux-aarch64.tar.gz -C dist {{binary}}-linux-aarch64
    @echo "Packaged: dist/{{binary}}-linux-aarch64.tar.gz"

# Package macOS x86-64 binary into a .tar.gz
package-macos-x86: dist-macos-x86
    mkdir -p dist
    cp target/x86_64-apple-darwin/release/{{binary}} dist/{{binary}}-macos-x86_64
    tar -czf dist/{{binary}}-macos-x86_64.tar.gz -C dist {{binary}}-macos-x86_64
    @echo "Packaged: dist/{{binary}}-macos-x86_64.tar.gz"

# Package macOS ARM64 (Apple Silicon) binary into a .tar.gz
package-macos-arm64: dist-macos-arm64
    mkdir -p dist
    cp target/aarch64-apple-darwin/release/{{binary}} dist/{{binary}}-macos-aarch64
    tar -czf dist/{{binary}}-macos-aarch64.tar.gz -C dist {{binary}}-macos-aarch64
    @echo "Packaged: dist/{{binary}}-macos-aarch64.tar.gz"

# Package Windows x86-64 binary into a .tar.gz
package-windows: dist-windows
    mkdir -p dist
    cp target/x86_64-pc-windows-gnu/release/{{binary}}.exe dist/{{binary}}-windows-x86_64.exe
    tar -czf dist/{{binary}}-windows-x86_64.tar.gz -C dist {{binary}}-windows-x86_64.exe
    @echo "Packaged: dist/{{binary}}-windows-x86_64.tar.gz"

# Build and package all targets into .tar.gz archives at project root
package-all: package-linux package-linux-arm64 package-macos-x86 package-macos-arm64 package-windows
    @echo "All archives in dist/"
