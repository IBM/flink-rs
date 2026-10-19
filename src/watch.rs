// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

/// In-place job watcher using ratatui + crossterm.
///
/// Renders a live-updating table showing the job's current state,
/// duration, and task breakdown. Exits when the job reaches a
/// terminal state (FINISHED, FAILED, CANCELED) or the user presses
/// 'q' / Ctrl-C.
use crate::{api, client::FlinkClient, output::table};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

const TERMINAL_STATES: &[&str] = &["FINISHED", "FAILED", "CANCELED", "CANCELLING"];

pub async fn run(client: &FlinkClient, job_id: &str, interval_secs: u64) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick = Duration::from_secs(interval_secs);
    let mut last_tick = Instant::now();
    let mut last_details: Option<crate::models::jobs::JobDetails> = None;
    let mut last_err: Option<String> = None;

    loop {
        // Poll for keyboard events (non-blocking).
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            let quit = key.code == KeyCode::Char('q')
                || key.code == KeyCode::Esc
                || (key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL));
            if quit {
                break;
            }
        }

        // Refresh on tick.
        if last_tick.elapsed() >= tick {
            last_tick = Instant::now();
            match api::jobs::details(client, job_id).await {
                Ok(details) => {
                    last_err = None;
                    let terminal_state = details
                        .state
                        .as_deref()
                        .map(|s| TERMINAL_STATES.contains(&s))
                        .unwrap_or(false);
                    last_details = Some(details);
                    if terminal_state {
                        // One final render, then exit.
                        terminal.draw(|f| render(f, job_id, &last_details, &last_err, interval_secs))?;
                        break;
                    }
                }
                Err(e) => {
                    last_err = Some(e.to_string());
                }
            }
        }

        terminal.draw(|f| render(f, job_id, &last_details, &last_err, interval_secs))?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Print final state to stdout after leaving alt screen.
    if let Some(d) = &last_details {
        println!("\nFinal state:");
        table::print_job_details(d);
    }

    Ok(())
}

fn render(
    f: &mut ratatui::Frame,
    job_id: &str,
    details: &Option<crate::models::jobs::JobDetails>,
    err: &Option<String>,
    interval_secs: u64,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title bar
            Constraint::Min(10),   // details table
            Constraint::Length(3), // footer
        ])
        .split(f.area());

    // ── title ─────────────────────────────────────────────────────────────
    let name = details
        .as_ref()
        .and_then(|d| d.name.as_deref())
        .unwrap_or("…");
    let title_text = Line::from(vec![
        Span::raw("Watching job "),
        Span::styled(job_id, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("  ({name})")),
    ]);
    let title = Paragraph::new(title_text)
        .block(Block::default().borders(Borders::ALL).title("flink watch"));
    f.render_widget(title, chunks[0]);

    // ── body ──────────────────────────────────────────────────────────────
    if let Some(err_msg) = err {
        let para = Paragraph::new(err_msg.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(para, chunks[1]);
    } else if let Some(d) = details {
        let state = d.state.as_deref().unwrap_or("UNKNOWN");
        let state_color = match state {
            "RUNNING" => Color::Green,
            "FINISHED" => Color::Blue,
            "FAILED" => Color::Red,
            "CANCELED" | "CANCELLING" => Color::Yellow,
            _ => Color::White,
        };

        let duration_str = table::format_duration_ms(d.duration);
        let tasks_total = d.tasks.as_ref().and_then(|t| t.total).map(|v| v.to_string()).unwrap_or_else(|| "—".into());
        let tasks_running = d.tasks.as_ref().and_then(|t| t.running).map(|v| v.to_string()).unwrap_or_else(|| "—".into());
        let tasks_finished = d.tasks.as_ref().and_then(|t| t.finished).map(|v| v.to_string()).unwrap_or_else(|| "—".into());
        let tasks_failed = d.tasks.as_ref().and_then(|t| t.failed).map(|v| v.to_string()).unwrap_or_else(|| "—".into());
        let job_type = d.job_type.as_deref().unwrap_or("—");

        let rows: Vec<Row> = vec![
            Row::new(vec!["Status", state])
                .style(Style::default().fg(state_color).add_modifier(Modifier::BOLD)),
            Row::new(vec!["Duration", &duration_str]),
            Row::new(vec!["Type", job_type]),
            Row::new(vec!["Tasks Total", &tasks_total]),
            Row::new(vec!["Tasks Running", &tasks_running]),
            Row::new(vec!["Tasks Finished", &tasks_finished]),
            Row::new(vec!["Tasks Failed", &tasks_failed]),
        ];

        let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
        let table_widget = Table::new(rows, widths)
            .header(
                Row::new(vec!["Field", "Value"])
                    .style(Style::default().add_modifier(Modifier::UNDERLINED)),
            )
            .block(Block::default().borders(Borders::ALL).title("Job Details"));
        f.render_widget(table_widget, chunks[1]);
    } else {
        let para = Paragraph::new("Fetching…")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(para, chunks[1]);
    }

    // ── footer ────────────────────────────────────────────────────────────
    let footer = Paragraph::new(format!(
        " q / Ctrl-C: quit  │  refreshes every {interval_secs}s"
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
