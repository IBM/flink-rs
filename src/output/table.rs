// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use crate::models::{
    cluster::{ClusterOverview, ConfigEntry, TaskManagerInfo},
    jars::JarInfo,
    jobs::JobOverview,
};
use colored::Colorize;
use comfy_table::{Cell, Color, Table, presets::UTF8_FULL_CONDENSED};

// ─── helpers ────────────────────────────────────────────────────────────────

fn str(opt: &Option<String>) -> &str {
    opt.as_deref().unwrap_or("—")
}

fn i32_str(opt: Option<i32>) -> String {
    opt.map(|v| v.to_string()).unwrap_or_else(|| "—".into())
}

pub fn format_status(state: &str) -> colored::ColoredString {
    match state {
        "RUNNING" => state.green().bold(),
        "FINISHED" => state.blue(),
        "FAILED" => state.red().bold(),
        "CANCELED" | "CANCELLING" => state.yellow(),
        "CREATED" => state.white(),
        _ => state.normal(),
    }
}

pub fn format_duration_ms(ms: Option<i64>) -> String {
    match ms {
        None | Some(0) | Some(-1) => "—".into(),
        Some(ms) => {
            let secs = ms / 1000;
            let mins = secs / 60;
            let hours = mins / 60;
            format!("{:02}:{:02}:{:02}", hours, mins % 60, secs % 60)
        }
    }
}

fn new_table() -> Table {
    let mut t = Table::new();
    t.load_preset(UTF8_FULL_CONDENSED);
    t
}

// ─── cluster overview ───────────────────────────────────────────────────────

pub fn print_overview(o: &ClusterOverview) {
    let version = o.flink_version.as_deref().unwrap_or("unknown");
    let commit = o.flink_commit.as_deref().unwrap_or("unknown");
    println!("Flink {}  commit {}\n", version.bold(), commit);

    let mut t = new_table();
    t.set_header(vec!["Metric", "Value"]);
    let rows: &[(&str, String)] = &[
        ("Jobs Running", i32_str(o.jobs_running)),
        ("Jobs Finished", i32_str(o.jobs_finished)),
        ("Jobs Cancelled", i32_str(o.jobs_cancelled)),
        ("Jobs Failed", i32_str(o.jobs_failed)),
        ("Task Managers", i32_str(o.taskmanagers)),
        ("Slots Total", i32_str(o.slots_total)),
        ("Slots Available", i32_str(o.slots_available)),
    ];
    for (k, v) in rows {
        t.add_row(vec![*k, v.as_str()]);
    }
    println!("{t}");

    let healthy = o.taskmanagers.unwrap_or(0) > 0 && o.slots_available.unwrap_or(0) >= 0;
    if healthy {
        println!("\nStatus: {}", "HEALTHY ✓".green().bold());
    } else {
        println!("\nStatus: {}", "UNHEALTHY ✗".red().bold());
    }
}

// ─── cluster config ─────────────────────────────────────────────────────────

pub fn print_config_table(entries: &[ConfigEntry]) {
    let mut t = new_table();
    t.set_header(vec!["Key", "Value"]);
    for e in entries {
        t.add_row(vec![str(&e.key), str(&e.value)]);
    }
    println!("{t}");
}

// ─── task managers ──────────────────────────────────────────────────────────

pub fn print_taskmanager_list(tms: &[TaskManagerInfo]) {
    if tms.is_empty() {
        println!("No task managers found.");
        return;
    }
    let mut t = new_table();
    t.set_header(vec!["ID", "Path", "Slots", "Free Slots"]);
    for tm in tms {
        t.add_row(vec![
            str(&tm.id),
            str(&tm.path),
            &i32_str(tm.slots_number),
            &i32_str(tm.free_slots),
        ]);
    }
    println!("{t}");
}

// ─── jobs ────────────────────────────────────────────────────────────────────

pub fn print_job_list(jobs: &[JobOverview]) {
    if jobs.is_empty() {
        println!("No jobs found.");
        return;
    }
    let mut t = new_table();
    t.set_header(vec!["Job ID", "Name", "Status", "Duration"]);
    for j in jobs {
        let state = j.state.as_deref().unwrap_or("UNKNOWN");
        let colored_state = format_status(state).to_string();
        t.add_row(vec![
            Cell::new(str(&j.jid)),
            Cell::new(str(&j.name)),
            Cell::new(&colored_state).fg(status_color(state)),
            Cell::new(format_duration_ms(j.duration)),
        ]);
    }
    println!("{t}");
}

fn status_color(state: &str) -> Color {
    match state {
        "RUNNING" => Color::Green,
        "FINISHED" => Color::Blue,
        "FAILED" => Color::Red,
        _ => Color::White,
    }
}

pub fn print_job_details(j: &crate::models::jobs::JobDetails) {
    let mut t = new_table();
    t.set_header(vec!["Field", "Value"]);
    let state = j.state.as_deref().unwrap_or("UNKNOWN");
    t.add_row(vec!["Job ID", str(&j.jid)]);
    t.add_row(vec!["Name", str(&j.name)]);
    t.add_row(vec!["Status", &format_status(state).to_string()]);
    t.add_row(vec!["Type", str(&j.job_type)]);
    t.add_row(vec!["Duration", &format_duration_ms(j.duration)]);
    if let Some(tasks) = &j.tasks {
        t.add_row(vec!["Tasks Total", &i32_str(tasks.total)]);
        t.add_row(vec!["Tasks Running", &i32_str(tasks.running)]);
        t.add_row(vec!["Tasks Finished", &i32_str(tasks.finished)]);
        t.add_row(vec!["Tasks Failed", &i32_str(tasks.failed)]);
    }
    println!("{t}");
}

// ─── jars ────────────────────────────────────────────────────────────────────

pub fn print_jar_list(jars: &[JarInfo]) {
    if jars.is_empty() {
        println!("No jars uploaded.");
        return;
    }
    let mut t = new_table();
    t.set_header(vec!["JAR ID", "Name", "Entry Class"]);
    for j in jars {
        let entry = j
            .entry
            .as_ref()
            .and_then(|e| e.first())
            .and_then(|e| e.name.as_deref())
            .unwrap_or("—");
        t.add_row(vec![str(&j.id), str(&j.name), entry]);
    }
    println!("{t}");
}
