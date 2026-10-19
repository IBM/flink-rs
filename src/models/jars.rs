// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JarList {
    pub files: Vec<JarInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JarInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub uploaded: Option<i64>,
    pub entry: Option<Vec<JarEntry>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JarEntry {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JarUploadResponse {
    pub filename: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JarRunRequestBody {
    #[serde(rename = "entryClass", skip_serializing_if = "Option::is_none")]
    pub entry_class: Option<String>,
    #[serde(rename = "programArgsList", skip_serializing_if = "Option::is_none")]
    pub program_args_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(rename = "savepointPath", skip_serializing_if = "Option::is_none")]
    pub savepoint_path: Option<String>,
    #[serde(rename = "allowNonRestoredState", skip_serializing_if = "Option::is_none")]
    pub allow_non_restored_state: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JarRunResponse {
    #[serde(rename = "jobid")]
    pub job_id: Option<String>,
}
