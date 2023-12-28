use std::io::{self, Stdout};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use miette::IntoDiagnostic;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub mod input;
pub mod table;

pub fn prepare_app() -> miette::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().into_diagnostic()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).into_diagnostic()?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).into_diagnostic()?;

    Ok(terminal)
}

pub fn finalize_app(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> miette::Result<()> {
    disable_raw_mode().into_diagnostic()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .into_diagnostic()?;
    terminal.show_cursor().into_diagnostic()?;

    Ok(())
}
