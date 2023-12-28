use std::process::Command;

use miette::{IntoDiagnostic, WrapErr};
use ratatui::layout::Constraint;
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Cell;

use crate::components::table::{run_app, Table};
use crate::components::{finalize_app, prepare_app};
use crate::parsers::package::pacman::packages;

pub fn list(_args: ciri::args::package::List) -> miette::Result<()> {
    let out = Command::new("pacman")
        .args(["-Q", "-i"])
        .output()
        .into_diagnostic()
        .wrap_err("Command pacman not found")?;

    let out = String::from_utf8(out.stdout).unwrap();
    let packages = packages(out.trim());

    let f_packages = packages
        .iter()
        .enumerate()
        .map(|(i, v)| {
            vec![
                i.to_string(),
                v.name.clone(),
                v.version.clone(),
                v.description.clone(),
                v.licenses.join(", ").clone(),
                v.url.clone().unwrap_or("".to_owned()),
                v.installed_size.clone(),
                v.install_date.clone(),
            ]
        })
        .collect::<Vec<Vec<String>>>();

    let headers = vec![
        "ID",
        "Package name",
        "Version",
        "Description",
        "Licenses",
        "URL",
        "Installed Size",
        "Install Date",
    ];

    let widths = vec![
        Constraint::Percentage(2),
        Constraint::Percentage(12),
        Constraint::Percentage(14),
        Constraint::Percentage(23),
        Constraint::Percentage(14),
        Constraint::Percentage(14),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
    ];

    let app = Table::new(
        "System Package List",
        None,
        headers,
        f_packages,
        widths,
        Some(|cell, multiline_cell, j| {
            if j == 1 {
                Cell::from(cell.clone()).bold()
            } else if j == 4 {
                Cell::from(
                    Line::styled(multiline_cell, Style::default().green())
                        .alignment(ratatui::layout::Alignment::Center),
                )
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
