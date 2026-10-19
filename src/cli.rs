// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use crate::config::OutputFormat;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "flink",
    about = "Apache Flink CLI — interact with a running cluster via the REST API",
    version,
    propagate_version = true
)]
pub struct Cli {
    /// Flink JobManager REST URL (overrides config file)
    #[arg(long, global = true, env = "FLINK_HOST")]
    pub host: Option<String>,

    /// Output format: table (default) or json
    #[arg(long, global = true, value_enum)]
    pub output: Option<OutputFormat>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage saved CLI configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Cluster health and status
    Cluster {
        #[command(subcommand)]
        action: ClusterAction,
    },

    /// Task manager information
    Taskmanagers {
        #[command(subcommand)]
        action: TaskmanagerAction,
    },

    /// Inspect and manage jobs
    Jobs {
        #[command(subcommand)]
        action: JobAction,
    },

    /// Manage uploaded JARs and submit jobs
    Jars {
        #[command(subcommand)]
        action: JarAction,
    },
}

// ─── config subcommands ──────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current resolved configuration
    Show,
    /// Set a configuration value
    Set {
        /// Key to set (host, timeout-secs)
        key: String,
        /// Value to set
        value: String,
    },
    /// Show the path to the config file
    Path,
}

// ─── cluster subcommands ─────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum ClusterAction {
    /// Show cluster overview and health status
    Overview,
    /// Show cluster configuration
    Config,
    /// Shut down the cluster (⚠ destructive)
    Shutdown {
        /// Confirm you mean it
        #[arg(long)]
        yes: bool,
    },
}

// ─── taskmanager subcommands ─────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum TaskmanagerAction {
    /// List all task managers
    List,
}

// ─── job subcommands ─────────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum JobAction {
    /// List all jobs
    List,
    /// Show details for a single job
    Get {
        /// Job ID (32-char hex)
        job_id: String,
    },
    /// Cancel a running job
    Cancel {
        /// Job ID
        job_id: String,
    },
    /// Get the execution result for a finished job
    Result {
        /// Job ID
        job_id: String,
    },
    /// Watch a job in-place, refreshing every --interval seconds
    Watch {
        /// Job ID
        job_id: String,
        /// Refresh interval in seconds
        #[arg(long, default_value = "3")]
        interval: u64,
    },
}

// ─── jar subcommands ─────────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum JarAction {
    /// List all uploaded JARs
    List,
    /// Upload a JAR to the cluster
    Upload {
        /// Path to the JAR file
        jar: PathBuf,
        /// Print only the JAR ID (suitable for shell substitution)
        #[arg(long)]
        id_only: bool,
    },
    /// Run a previously uploaded JAR
    Run {
        /// JAR ID (from `jars list`)
        jar_id: String,
        /// Fully-qualified entry class name
        #[arg(long)]
        entry_class: Option<String>,
        /// Desired parallelism
        #[arg(long)]
        parallelism: Option<i32>,
        /// Comma-separated program arguments
        #[arg(long)]
        args: Option<String>,
        /// Savepoint path to restore from
        #[arg(long)]
        savepoint: Option<String>,
        /// Return immediately with job ID instead of waiting for completion
        #[arg(long)]
        no_wait: bool,
    },
    /// Upload a JAR and immediately run it (combines upload + run)
    Submit {
        /// Path to the JAR file
        jar: PathBuf,
        /// Fully-qualified entry class name
        #[arg(long)]
        entry_class: Option<String>,
        /// Desired parallelism
        #[arg(long)]
        parallelism: Option<i32>,
        /// Comma-separated program arguments
        #[arg(long)]
        args: Option<String>,
        /// Return immediately with job ID instead of waiting for completion
        #[arg(long)]
        no_wait: bool,
    },
    /// Delete an uploaded JAR
    Delete {
        /// JAR ID
        jar_id: String,
    },
}
