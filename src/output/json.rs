// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

pub fn print_json<T: Serialize>(value: &T) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).unwrap_or_default()
    );
}
