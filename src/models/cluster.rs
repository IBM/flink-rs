// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterOverview {
    #[serde(rename = "flink-version")]
    pub flink_version: Option<String>,
    #[serde(rename = "flink-commit")]
    pub flink_commit: Option<String>,
    #[serde(rename = "jobs-running")]
    pub jobs_running: Option<i32>,
    #[serde(rename = "jobs-finished")]
    pub jobs_finished: Option<i32>,
    #[serde(rename = "jobs-cancelled")]
    pub jobs_cancelled: Option<i32>,
    #[serde(rename = "jobs-failed")]
    pub jobs_failed: Option<i32>,
    #[serde(rename = "taskmanagers")]
    pub taskmanagers: Option<i32>,
    #[serde(rename = "taskmanagers-blocked")]
    pub taskmanagers_blocked: Option<i32>,
    #[serde(rename = "slots-total")]
    pub slots_total: Option<i32>,
    #[serde(rename = "slots-available")]
    pub slots_available: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigEntry {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskManagerList {
    pub taskmanagers: Vec<TaskManagerInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskManagerInfo {
    pub id: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "dataPort")]
    pub data_port: Option<i32>,
    #[serde(rename = "slotsNumber")]
    pub slots_number: Option<i32>,
    #[serde(rename = "freeSlots")]
    pub free_slots: Option<i32>,
    #[serde(rename = "totalResource")]
    pub total_resource: Option<TaskManagerResource>,
    #[serde(rename = "freeResource")]
    pub free_resource: Option<TaskManagerResource>,
    pub hardware: Option<Hardware>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskManagerResource {
    #[serde(rename = "cpuCores")]
    pub cpu_cores: Option<f64>,
    #[serde(rename = "taskHeapMemory")]
    pub task_heap_memory: Option<i64>,
    #[serde(rename = "taskOffHeapMemory")]
    pub task_off_heap_memory: Option<i64>,
    #[serde(rename = "managedMemory")]
    pub managed_memory: Option<i64>,
    #[serde(rename = "networkMemory")]
    pub network_memory: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hardware {
    #[serde(rename = "cpuCores")]
    pub cpu_cores: Option<i32>,
    #[serde(rename = "physicalMemory")]
    pub physical_memory: Option<i64>,
    #[serde(rename = "freeMemory")]
    pub free_memory: Option<i64>,
    #[serde(rename = "managedMemory")]
    pub managed_memory: Option<i64>,
}
