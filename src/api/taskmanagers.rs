// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::FlinkClient,
    models::cluster::TaskManagerList,
};
use anyhow::Result;

pub async fn list(client: &FlinkClient) -> Result<TaskManagerList> {
    client.get("/taskmanagers").await
}
