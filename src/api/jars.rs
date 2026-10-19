// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::FlinkClient,
    models::jars::{JarList, JarRunRequestBody, JarRunResponse, JarUploadResponse},
};
use anyhow::{Context, Result};
use std::path::Path;

pub async fn list(client: &FlinkClient) -> Result<JarList> {
    client.get("/jars").await
}

pub async fn upload(client: &FlinkClient, jar_path: &Path) -> Result<JarUploadResponse> {
    let filename = jar_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("job.jar")
        .to_owned();
    let bytes = std::fs::read(jar_path)
        .with_context(|| format!("reading jar at {}", jar_path.display()))?;
    client.upload_jar("/jars/upload", bytes, filename).await
}

pub async fn run(
    client: &FlinkClient,
    jar_id: &str,
    body: &JarRunRequestBody,
) -> Result<JarRunResponse> {
    client
        .post_json(&format!("/jars/{jar_id}/run"), body)
        .await
}

pub async fn delete(client: &FlinkClient, jar_id: &str) -> Result<()> {
    client.delete(&format!("/jars/{jar_id}")).await
}
