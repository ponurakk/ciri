use std::io;
use std::process::Command;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::components::table::{run_app, App};
use crate::parsers::package::pacman::packages;

pub fn list(_args: ciri::args::package::List) -> Result<(), io::Error> {
    let out = Command::new("pacman").args(["-Q", "-i"]).output().unwrap();
    let out = String::from_utf8(out.stdout).unwrap();
    let packages = packages(out.trim());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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

    let mut app = App::new(f_packages);
    app.state.select(Some(0));
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}
