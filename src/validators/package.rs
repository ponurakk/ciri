use miette::IntoDiagnostic;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Cell;

use crate::components::table::{run_app, Table};
use crate::components::{finalize_app, prepare_app};
use crate::PackageManagers;

pub fn find() -> miette::Result<()> {
    let app = Table::new(
        "Packages Health",
        Some(|size| Rect {
            width: Constraint::Percentage(35).apply(size.width),
            height: size.height - Constraint::Length(3).apply(3),
            ..Default::default()
        }),
        vec!["Name", "Status"],
        PackageManagers::to_vec()
            .iter()
            .flat_map(|name| vec![vec![name.to_string(), check(name)]])
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

pub fn check(name: &str) -> String {
    let pkg = which::which(name).into_diagnostic();

    if pkg.is_ok() {
        "🗹 ".to_string()
    } else {
        "🗷 ".to_string()
    }
}
