use std::io::{self, Write};
use std::process::Command;

use comfy_table::{modifiers, presets, Attribute, Cell, Color, Table};
use crossterm::event::{Event, KeyCode};
use crossterm::{cursor, terminal, ExecutableCommand};

use crate::parsers::package::pacman::packages;

pub fn list(_args: ciri::args::package::List) -> Result<(), io::Error> {
    let out = Command::new("pacman").args(["-Q", "-i"]).output().unwrap();
    let out = String::from_utf8(out.stdout).unwrap();
    let packages = packages(out.trim());

    let rows: Vec<Vec<Cell>> = packages
        .iter()
        .map(|v| {
            vec![
                Cell::new(&v.name).add_attribute(Attribute::Dim),
                Cell::new(&v.version),
                Cell::new(&v.description),
                Cell::new(&v.licenses.join(", ")).fg(Color::Green),
                Cell::new(v.url.clone().unwrap_or("".to_owned())),
                Cell::new(&v.installed_size),
                Cell::new(&v.install_date),
            ]
        })
        .collect();

    // Clear screen
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // Main loop for rendering and handling input
    let mut offset = 0;
    loop {
        terminal::disable_raw_mode()?;
        // Rendering the table with scrolling
        render_table(&rows, offset)?;
        terminal::enable_raw_mode()?;

        // Handle input
        if let Some(event) = crossterm::event::read().ok() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Char('q') => break, // Exit on 'q'
                    KeyCode::Down | KeyCode::Char('j') => {
                        if offset < rows.len() - 1 {
                            offset += 1;
                        }
                    }
                    KeyCode::PageDown => {
                        if offset < rows.len() - 10 {
                            offset += 10;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if offset >= 1 {
                            offset -= 1;
                        }
                    }
                    KeyCode::PageUp => {
                        if offset >= 10 {
                            offset -= 10;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn render_table(rows: &Vec<Vec<Cell>>, offset: usize) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut table = Table::new();
    let height = (terminal::size().unwrap().1 / 6) as usize;

    table
        .set_header(vec![
            Cell::new("Package name").add_attribute(Attribute::Bold),
            Cell::new("Version").add_attribute(Attribute::Bold),
            Cell::new("Description").add_attribute(Attribute::Bold),
            Cell::new("Licenses").add_attribute(Attribute::Bold),
            Cell::new("URL").add_attribute(Attribute::Bold),
            Cell::new("Installed Size").add_attribute(Attribute::Bold),
            Cell::new("Install Date").add_attribute(Attribute::Bold),
        ])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(modifiers::UTF8_ROUND_CORNERS)
        .apply_modifier(modifiers::UTF8_SOLID_INNER_BORDERS);

    let mut new_offset = offset;

    if offset + height >= rows.len() {
        new_offset = offset - height;
    }

    let filtered_rows: Vec<Vec<Cell>> = rows
        .get(0 + new_offset..=height + new_offset)
        .unwrap()
        .to_vec();
    table.add_rows(filtered_rows);

    stdout.flush()?;
    println!("{}", table);
    println!(
        "q(quit), k(up), j(down), pgUp(up {0}), pgDown(down {0})",
        height
    );
    Ok(())
}
