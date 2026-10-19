## Contributing In General

Our project welcomes external contributions. If you have an itch, please feel free to scratch it.

To contribute code or documentation, please submit a [pull request](https://github.com/IBM/flink-rs/pulls).

A good way to familiarize yourself with the codebase and contribution process is to look for and tackle low-hanging fruit in the [issue tracker](https://github.com/IBM/flink-rs/issues).
Before embarking on a more ambitious contribution, please quickly [get in touch](#communication) with us.

**Note: We appreciate your effort, and want to avoid a situation where a contribution
requires extensive rework (by you or by us), sits in backlog for a long time, or
cannot be accepted at all!**

### Proposing new features

If you would like to implement a new feature, please [raise an issue](https://github.com/IBM/flink-rs/issues) before sending a pull request so the feature can be discussed. This is to avoid you wasting your valuable time working on a feature that the project developers are not interested in accepting into the code base.

### Fixing bugs

If you would like to fix a bug, please [raise an issue](https://github.com/IBM/flink-rs/issues) before sending a pull request so it can be tracked.

### Merge approval

The project maintainers use LGTM (Looks Good To Me) in comments on the code review to indicate acceptance. A change requires LGTMs from two of the maintainers of each component affected.

For a list of the maintainers, see the [MAINTAINERS.md](MAINTAINERS.md) page.

## Legal

Each source file must include a license header for the Apache Software License 2.0. Using the SPDX format is the simplest approach.

```
/*
Copyright IBM Corp. 2025 All Rights Reserved.

SPDX-License-Identifier: Apache-2.0
*/
```

We use the [Developer's Certificate of Origin 1.1 (DCO)](https://github.com/hyperledger/fabric/blob/master/docs/source/DCO1.1.txt) to manage code contributions.

When submitting a patch for review, the developer must include a sign-off statement in the commit message:

```
Signed-off-by: John Doe <john.doe@example.com>
```

You can include this automatically when you commit:

```
git commit -s
```

## Communication

Open an issue at [github.com/IBM/flink-rs/issues](https://github.com/IBM/flink-rs/issues).

## Setup

Prerequisites:

- [Rust](https://rustup.rs/) stable toolchain
- [`just`](https://github.com/casey/just) — `cargo install just`

```bash
git clone https://github.com/IBM/flink-rs
cd flink-rs
cargo build
```

For cross-compilation targets see the [Cross-compile targets](README.md#cross-compile-targets) section of the README.

## Testing

```bash
just ci       # fmt check + clippy + build + test (full CI gate)
cargo test    # unit tests only
```

Before submitting a PR, run `just ci` and make sure it passes clean.

## Coding style guidelines

- Format with `cargo fmt` (enforced in CI).
- Lint with `cargo clippy -- -D warnings` (enforced in CI).
- Keep each source file's SPDX header intact.
- Prefer `anyhow::Result` for error propagation; avoid `unwrap()` in non-test code.
