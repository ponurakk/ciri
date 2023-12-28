use std::path::PathBuf;

use miette::IntoDiagnostic;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Cell;

use crate::components::table::{run_app, Table};
use crate::components::{finalize_app, prepare_app};

pub fn find() -> miette::Result<()> {
    let packages = vec![
        "bun", "cargo", "clang", "clang++", "composer", "dart", "deno", "flutter", "g++", "gcc",
        "go", "gradle", "groovy", "java", "kotlin", "lua", "maven", "node", "npm", "perl", "php",
        "pip", "pnpm", "python", "ruby", "scala", "swift", "yarn", "zig",
    ];
    let app = Table::new(
        "Packages Health",
        Some(|size| Rect {
            width: Constraint::Percentage(35).apply(size.width),
            height: size.height - Constraint::Length(3).apply(3),
            ..Default::default()
        }),
        vec!["Name", "Status"],
        packages
            .iter()
            .flat_map(|name| vec![vec![name.to_string(), format_check(check(name).is_ok())]])
            .collect::<Vec<Vec<String>>>(),
        vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        Some(|cell, multiline_cell, j| {
            if j == 1 {
                if cell.contains("🗹 ") {
                    Cell::from(
                        Line::styled(cell.clone(), Style::default().green())
                            .alignment(ratatui::layout::Alignment::Center),
                    )
                } else if cell.contains("🗷 ") {
                    Cell::from(
                        Line::styled(cell.clone(), Style::default().red())
                            .alignment(ratatui::layout::Alignment::Center),
                    )
                } else {
                    Cell::from(cell.clone())
                }
            } else {
                Cell::from(multiline_cell)
            }
        }),
    );

    let mut terminal = prepare_app()?;
    run_app(app, &mut terminal)?;
    finalize_app(terminal)?;

    Ok(())
}

pub fn check(name: &str) -> miette::Result<PathBuf> {
    which::which(name).into_diagnostic()
}

pub fn format_check(bool: bool) -> String {
    if bool {
        "🗹 ".to_string()
    } else {
        "🗷 ".to_string()
    }
}
