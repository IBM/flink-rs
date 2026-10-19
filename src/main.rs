// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

mod api;
mod cli;
mod client;
mod config;
mod models;
mod output;
mod watch;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{ClusterAction, Commands, ConfigAction, JarAction, JobAction, TaskmanagerAction};
use client::FlinkClient;
use colored::Colorize;
use config::OutputFormat;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let cfg = config::resolve(cli.host, cli.output);

    if let Err(e) = run(cli.command, cfg).await {
        eprintln!("{} {e:#}", "error:".red().bold());
        std::process::exit(1);
    }
}

async fn run(command: Commands, cfg: config::Config) -> Result<()> {
    match command {
        // ── config ────────────────────────────────────────────────────────
        Commands::Config { action } => match action {
            ConfigAction::Show => {
                println!("Config file: {}", config::Config::path().display());
                println!("host        = {}", cfg.host);
                println!("timeout     = {}s", cfg.timeout_secs);
                println!("output      = {:?}", cfg.output);
            }
            ConfigAction::Path => {
                println!("{}", config::Config::path().display());
            }
            ConfigAction::Set { key, value } => {
                let mut saved = config::Config::load();
                match key.as_str() {
                    "host" => saved.host = value,
                    "timeout-secs" => {
                        saved.timeout_secs = value
                            .parse()
                            .map_err(|_| anyhow::anyhow!("timeout-secs must be a number"))?;
                    }
                    "output" => {
                        saved.output = match value.as_str() {
                            "table" => OutputFormat::Table,
                            "json" => OutputFormat::Json,
                            _ => bail!("output must be 'table' or 'json'"),
                        };
                    }
                    _ => {
                        bail!("unknown config key '{key}'. Valid keys: host, timeout-secs, output")
                    }
                }
                saved.save()?;
                println!("Saved.");
            }
        },

        // ── cluster ───────────────────────────────────────────────────────
        Commands::Cluster { action } => {
            let client = make_client(&cfg)?;
            match action {
                ClusterAction::Overview => {
                    let overview = api::cluster::overview(&client).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&overview);
                    } else {
                        output::print_overview(&overview);
                    }
                }
                ClusterAction::Config => {
                    let entries = api::cluster::config(&client).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&entries);
                    } else {
                        output::print_config_table(&entries);
                    }
                }
                ClusterAction::Shutdown { yes } => {
                    if !yes {
                        bail!("Pass --yes to confirm cluster shutdown.");
                    }
                    api::cluster::shutdown(&client).await?;
                    println!("{}", "Cluster shutdown initiated.".yellow());
                }
            }
        }

        // ── taskmanagers ──────────────────────────────────────────────────
        Commands::Taskmanagers { action } => {
            let client = make_client(&cfg)?;
            match action {
                TaskmanagerAction::List => {
                    let list = api::taskmanagers::list(&client).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&list);
                    } else {
                        output::print_taskmanager_list(&list.taskmanagers);
                    }
                }
            }
        }

        // ── jobs ──────────────────────────────────────────────────────────
        Commands::Jobs { action } => {
            let client = make_client(&cfg)?;
            match action {
                JobAction::List => {
                    let overview = api::jobs::overview(&client).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&overview);
                    } else {
                        output::print_job_list(&overview.jobs);
                    }
                }
                JobAction::Get { job_id } => {
                    let details = api::jobs::details(&client, &job_id).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&details);
                    } else {
                        output::print_job_details(&details);
                    }
                }
                JobAction::Cancel { job_id } => {
                    api::jobs::cancel(&client, &job_id).await?;
                    println!("Cancellation requested for job {job_id}");
                }
                JobAction::Result { job_id } => {
                    let result = api::jobs::execution_result(&client, &job_id).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&result);
                    } else {
                        if let Some(r) = &result.result {
                            println!("Net runtime: {}ms", r.net_runtime.unwrap_or(0));
                            if let Some(acc) = &r.accumulator_results {
                                println!(
                                    "Accumulators:\n{}",
                                    serde_json::to_string_pretty(acc).unwrap_or_default()
                                );
                            }
                        } else {
                            println!("No result yet — job may still be running.");
                        }
                    }
                }
                JobAction::Watch { job_id, interval } => {
                    watch::run(&client, &job_id, interval).await?;
                }
            }
        }

        // ── jars ──────────────────────────────────────────────────────────
        Commands::Jars { action } => {
            let client = make_client(&cfg)?;
            match action {
                JarAction::List => {
                    let list = api::jars::list(&client).await?;
                    if cfg.output == OutputFormat::Json {
                        output::print_json(&list);
                    } else {
                        output::print_jar_list(&list.files);
                    }
                }
                JarAction::Upload { jar, id_only } => {
                    let resp = api::jars::upload(&client, &jar).await?;
                    let filename = resp.filename.as_deref().unwrap_or("unknown");
                    let jar_id = filename.rsplit('/').next().unwrap_or(filename);
                    if id_only {
                        println!("{jar_id}");
                    } else {
                        println!("Uploaded: {}", filename.green());
                        println!("JAR ID:   {jar_id}");
                    }
                }
                JarAction::Delete { jar_id } => {
                    api::jars::delete(&client, &jar_id).await?;
                    println!("Deleted {jar_id}");
                }
                JarAction::Run {
                    jar_id,
                    entry_class,
                    parallelism,
                    args,
                    savepoint,
                    no_wait,
                } => {
                    let job_id =
                        submit_jar(&client, &jar_id, entry_class, parallelism, args, savepoint)
                            .await?;
                    println!("Job submitted: {}", job_id.green().bold());
                    if !no_wait {
                        wait_for_job(&client, &job_id).await?;
                    }
                }
                JarAction::Submit {
                    jar,
                    entry_class,
                    parallelism,
                    args,
                    no_wait,
                } => {
                    // 1. Upload
                    println!("Uploading {}…", jar.display());
                    let upload_resp = api::jars::upload(&client, &jar).await?;
                    let filename = upload_resp.filename.unwrap_or_default();
                    let jar_id = filename.rsplit('/').next().unwrap_or(&filename).to_owned();
                    println!("Uploaded as JAR ID: {}", jar_id.dimmed());

                    // 2. Run
                    let job_id =
                        submit_jar(&client, &jar_id, entry_class, parallelism, args, None).await?;
                    println!("Job submitted: {}", job_id.green().bold());
                    if !no_wait {
                        wait_for_job(&client, &job_id).await?;
                    }
                }
            }
        }
    }

    Ok(())
}

// ─── helpers ─────────────────────────────────────────────────────────────────

fn make_client(cfg: &config::Config) -> Result<FlinkClient> {
    FlinkClient::new(&cfg.host, cfg.timeout_secs)
}

async fn submit_jar(
    client: &FlinkClient,
    jar_id: &str,
    entry_class: Option<String>,
    parallelism: Option<i32>,
    args: Option<String>,
    savepoint: Option<String>,
) -> Result<String> {
    let program_args_list = args.map(|a| a.split(',').map(str::to_owned).collect::<Vec<_>>());
    let body = models::jars::JarRunRequestBody {
        entry_class,
        parallelism,
        program_args_list,
        savepoint_path: savepoint,
        allow_non_restored_state: None,
    };
    let resp = api::jars::run(client, jar_id, &body).await?;
    resp.job_id
        .ok_or_else(|| anyhow::anyhow!("No job ID returned by Flink"))
}

/// Poll job status until it reaches a terminal state, printing updates.
async fn wait_for_job(client: &FlinkClient, job_id: &str) -> Result<()> {
    use std::time::Duration;
    use tokio::time::sleep;

    println!("Waiting for job {job_id} to complete…");
    loop {
        let state = api::jobs::status(client, job_id).await?;
        let colored = output::table::format_status(&state).to_string();
        print!("\r  Status: {colored:<20}");
        // flush stdout so the \r update is visible
        use std::io::Write;
        let _ = std::io::stdout().flush();

        match state.as_str() {
            "FINISHED" => {
                println!("\n{}", "Job finished successfully.".green().bold());
                return Ok(());
            }
            "FAILED" => {
                bail!("Job {job_id} FAILED");
            }
            "CANCELED" | "CANCELLING" => {
                println!("\n{}", "Job was cancelled.".yellow());
                return Ok(());
            }
            _ => {}
        }
        sleep(Duration::from_secs(3)).await;
    }
}
