// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result, bail};
use reqwest::{
    Client, StatusCode,
    multipart::{Form, Part},
};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Thin wrapper around reqwest that knows the Flink base URL.
#[derive(Clone)]
pub struct FlinkClient {
    pub base_url: String,
    inner: Client,
}

impl FlinkClient {
    pub fn new(base_url: impl Into<String>, timeout_secs: u64) -> Result<Self> {
        let inner = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .context("building HTTP client")?;
        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            inner,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path);
        let resp = self
            .inner
            .get(&url)
            .send()
            .await
            .with_context(|| connection_error(&self.base_url))?;
        self.deserialize(resp, &url).await
    }

    #[allow(dead_code)]
    pub async fn get_json(&self, path: &str) -> Result<serde_json::Value> {
        self.get::<serde_json::Value>(path).await
    }

    pub async fn post_json<B, T>(&self, path: &str, body: &B) -> Result<T>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        let url = self.url(path);
        let resp = self
            .inner
            .post(&url)
            .json(body)
            .send()
            .await
            .with_context(|| connection_error(&self.base_url))?;
        self.deserialize(resp, &url).await
    }

    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path);
        let resp = self
            .inner
            .post(&url)
            .send()
            .await
            .with_context(|| connection_error(&self.base_url))?;
        self.deserialize(resp, &url).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = self.url(path);
        let resp = self
            .inner
            .delete(&url)
            .send()
            .await
            .with_context(|| connection_error(&self.base_url))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("DELETE {url} failed {status}: {text}");
        }
        Ok(())
    }

    /// Upload a file as multipart/form-data to the given path.
    /// Returns the parsed response body.
    pub async fn upload_jar<T: DeserializeOwned>(
        &self,
        path: &str,
        file_bytes: Vec<u8>,
        filename: String,
    ) -> Result<T> {
        let url = self.url(path);
        let part = Part::bytes(file_bytes)
            .file_name(filename)
            .mime_str("application/x-java-archive")?;
        let form = Form::new().part("jarfile", part);
        let resp = self
            .inner
            .post(&url)
            .multipart(form)
            .send()
            .await
            .with_context(|| connection_error(&self.base_url))?;
        self.deserialize(resp, &url).await
    }

    async fn deserialize<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
        url: &str,
    ) -> Result<T> {
        let status = resp.status();
        if status == StatusCode::NOT_FOUND {
            bail!("Not found: {url}");
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            // Flink error bodies look like: {"errors":["..."]}
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&body)
                && let Some(errors) = v.get("errors").and_then(|e| e.as_array())
            {
                let msgs: Vec<_> = errors.iter().filter_map(|e| e.as_str()).collect();
                bail!("Flink API error ({}): {}", status, msgs.join("; "));
            }
            bail!("HTTP {} from {url}: {body}", status);
        }
        resp.json::<T>()
            .await
            .with_context(|| format!("parsing response from {url}"))
    }
}

fn connection_error(base_url: &str) -> String {
    format!(
        "Cannot connect to Flink at {base_url} — is the cluster running? (use --host to override)"
    )
}
