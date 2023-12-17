use std::process::Command;

use comfy_table::{modifiers, presets, Attribute, Cell, Table};

pub fn list(args: ciri::args::package::List) {
    info!("{} {}", args.installed, args.not_installed);
    let out = Command::new("pacman").args(["-Q", "-e"]).output().unwrap();
    let out = String::from_utf8(out.stdout).unwrap();
    let out_formated: Vec<&str> = out.trim().split("\n").collect();

    let mut table = Table::new();

    let rows: Vec<Vec<Cell>> = out_formated
        .iter()
        .map(|&v| {
            let (name, version) = v.split_at(v.find(' ').unwrap());
            vec![
                Cell::new(name).add_attribute(Attribute::Dim),
                Cell::new(version).add_attribute(Attribute::Encircled),
            ]
        })
        .collect();

    table.add_rows(rows);
    table
        .set_header(vec![
            Cell::new("Package name").add_attribute(Attribute::Bold),
            Cell::new("Version").add_attribute(Attribute::Bold),
        ])
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(modifiers::UTF8_ROUND_CORNERS)
        .apply_modifier(modifiers::UTF8_SOLID_INNER_BORDERS);

    println!("{}", table);
}
