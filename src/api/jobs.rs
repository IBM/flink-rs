// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::FlinkClient,
    models::jobs::{JobDetails, JobExecutionResult, JobsOverview},
};
use anyhow::Result;

pub async fn overview(client: &FlinkClient) -> Result<JobsOverview> {
    client.get("/jobs/overview").await
}

pub async fn details(client: &FlinkClient, job_id: &str) -> Result<JobDetails> {
    client.get(&format!("/jobs/{job_id}")).await
}

pub async fn cancel(client: &FlinkClient, job_id: &str) -> Result<()> {
    // DELETE /jobs/{jobid}?mode=cancel
    let url = format!("/jobs/{job_id}?mode=cancel");
    // cancel returns 202 with empty body — use serde_json::Value to absorb it
    let _: serde_json::Value = client
        .post_empty(&format!("/jobs/{job_id}/yarn-cancel"))
        .await
        .unwrap_or(serde_json::Value::Null);
    // Prefer PATCH cancel (Flink 1.x patch, 2.x uses DELETE)
    client.delete(&url).await
}

pub async fn execution_result(client: &FlinkClient, job_id: &str) -> Result<JobExecutionResult> {
    client
        .get(&format!("/jobs/{job_id}/execution-result"))
        .await
}

pub async fn status(client: &FlinkClient, job_id: &str) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct StatusBody {
        status: Option<String>,
    }
    let body: StatusBody = client.get(&format!("/jobs/{job_id}/status")).await?;
    Ok(body.status.unwrap_or_else(|| "UNKNOWN".into()))
}
