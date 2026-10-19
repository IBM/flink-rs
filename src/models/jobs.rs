// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobsOverview {
    pub jobs: Vec<JobOverview>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobOverview {
    pub jid: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "start-time")]
    pub start_time: Option<i64>,
    #[serde(rename = "end-time")]
    pub end_time: Option<i64>,
    pub duration: Option<i64>,
    pub tasks: Option<TaskCounts>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskCounts {
    pub total: Option<i32>,
    pub running: Option<i32>,
    pub finished: Option<i32>,
    pub failed: Option<i32>,
    pub canceled: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDetails {
    pub jid: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "start-time")]
    pub start_time: Option<i64>,
    #[serde(rename = "end-time")]
    pub end_time: Option<i64>,
    pub duration: Option<i64>,
    #[serde(rename = "isStoppable")]
    pub is_stoppable: Option<bool>,
    pub tasks: Option<TaskCounts>,
    #[serde(rename = "job-type")]
    pub job_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobExecutionResult {
    pub id: Option<String>,
    pub status: Option<ExecutionStatus>,
    #[serde(rename = "job-execution-result")]
    pub result: Option<JobResult>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionStatus {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobResult {
    pub id: Option<String>,
    #[serde(rename = "net-runtime")]
    pub net_runtime: Option<i64>,
    pub accumulator_results: Option<serde_json::Value>,
}
