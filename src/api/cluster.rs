// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::FlinkClient,
    models::cluster::{ClusterOverview, ConfigEntry},
};
use anyhow::Result;

pub async fn overview(client: &FlinkClient) -> Result<ClusterOverview> {
    client.get("/overview").await
}

pub async fn config(client: &FlinkClient) -> Result<Vec<ConfigEntry>> {
    client.get("/jobmanager/config").await
}

pub async fn shutdown(client: &FlinkClient) -> Result<()> {
    client.delete("/cluster").await
}
