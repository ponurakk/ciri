use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table as RatTable, TableState},
    Frame, Terminal,
};
use std::io;

use super::input::{ui as ui_input, Input, InputMode};

pub struct Table<'a> {
    /// Title of table
    title: &'a str,
    /// Size of bounding box. Can be relative to terminal size
    size: Option<fn(size: Rect) -> Rect>,
    state: TableState,
    /// Items to be parsed as rows
    items: Vec<Vec<String>>,
    /// List of maximum available height for rows
    max_heights: Vec<u16>,
    /// List of current height for rows
    current_heights: Vec<u16>,
    /// Table Headers
    headers: Vec<&'a str>,
    /// Table Rows
    rows: Vec<Row<'a>>,
    /// Widths of headers
    widths: Vec<Constraint>,
    /// Callback to execute on each row
    callback: Option<fn(content: &String, multiline_content: String, j: usize) -> Cell<'a>>,
    /// Input for filtering table
    input: Input,
}

impl<'a> Table<'a> {
    pub fn new(
        title: &'a str,
        table_size: Option<fn(size: Rect) -> Rect>,
        headers: Vec<&'a str>,
        items: Vec<Vec<String>>,
        widths: Vec<Constraint>,
        callback: Option<fn(content: &String, multiline_content: String, j: usize) -> Cell<'a>>,
    ) -> Table<'a> {
        if widths.len() != headers.len() {
            error!("Numbers of headers and their widths doesn't match");
        }

        Self {
            title,
            size: table_size,
            state: TableState::default(),
            max_heights: vec![1; items.len()],
            current_heights: vec![1; items.len()],
            items,
            headers,
            rows: Vec::new(),
            widths,
            callback,
            input: Input::new(Some(Box::new(|size: Rect| Rect {
                y: size.height - Constraint::Length(3).apply(3),
                width: size.width,
                height: size.height,
                ..Default::default()
            }))),
        }
    }

    pub fn next(&mut self, count: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.rows.len() - count {
                    if i.checked_sub(count).is_some() {
                        self.current_heights[i - count] = 1;
                    }
                    0
                } else {
                    self.current_heights[i] = 1;
                    i + count
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, count: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                self.current_heights[i] = 1;
                if i < count {
                    self.rows.len() - count
                } else {
                    i - count
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn expand(&mut self) {
        if let Some(i) = self.state.selected() {
            if self.current_heights[i] == 1 {
                self.current_heights[i] = self.max_heights[i];
            } else {
                self.current_heights[i] = 1;
            }
        }
    }
}

pub fn run_app<B: Backend>(mut app: Table, terminal: &mut Terminal<B>) -> miette::Result<()> {
    app.state.select(Some(0));

    let res = draw(terminal, app);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, mut app: Table) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('/') => {
                        app.input.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(1),
                    KeyCode::Up => app.previous(1),
                    KeyCode::PageDown => app.next(10),
                    KeyCode::PageUp => app.previous(10),
                    KeyCode::Enter => app.expand(),
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.input.submit_message(),
                    KeyCode::Char(to_insert) => {
                        app.input.enter_char(to_insert);
                        app.state.select(Some(0));
                        app.input.submit_message();
                    }
                    KeyCode::Backspace => {
                        app.input.delete_char();
                        app.state.select(Some(0));
                        app.input.submit_message()
                    }
                    KeyCode::Left => {
                        app.input.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.input.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.input.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut Table) {
    let width = f.size().width;
    let height = f.size().height;
    let rects = if let Some(size) = app.size {
        app.input.size_callback = Some(Box::new(move |size: Rect| Rect {
            y: size.height - Constraint::Length(3).apply(3),
            width: Constraint::Percentage(35).apply(width),
            height: height - Constraint::Length(3).apply(3),
            ..Default::default()
        }));

        Layout::default()
            .constraints([Constraint::Min(30)].as_ref())
            .split(size(f.size()))
    } else {
        app.input.size_callback = Some(Box::new(|size: Rect| Rect {
            y: size.height - Constraint::Length(3).apply(3),
            width: size.width,
            height: size.height,
            ..Default::default()
        }));

        Layout::default()
            .constraints([Constraint::Min(30)].as_ref())
            .split(Rect {
                x: f.size().x,
                y: f.size().y,
                width: f.size().width,
                height: f.size().height - Constraint::Length(3).apply(3),
            })
    };

    let select_style = Style::default().add_modifier(Modifier::REVERSED);

    let header = Row::new(app.headers.iter().map(|h| {
        Cell::from(
            Line::styled(
                *h,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            )
            .alignment(ratatui::layout::Alignment::Center),
        )
    }))
    .height(1)
    .bottom_margin(1);

    // Reset to full state
    app.max_heights = vec![1; app.items.len()];
    app.current_heights = vec![1; app.items.len()];

    let rows: Vec<Row<'_>> = app
        .items
        .iter()
        .enumerate()
        .filter(|(_, v)| v.iter().any(|str| str.contains(&app.input.message)))
        .map(|(i, item)| {
            let mut new_height = vec![];

            let cells: Vec<Cell> = item
                .iter()
                .enumerate()
                .map(|(j, cell)| {
                    let height = app.widths[j].apply(f.size().width);
                    let binding = cell.chars().collect::<Vec<_>>();
                    let vec = binding.chunks(height as usize);

                    let multiline_cell = vec
                        .clone()
                        .map(|v| v.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join("\n");

                    new_height.push(vec.len());

                    if let Some(callback) = app.callback {
                        callback(cell, multiline_cell, j)
                    } else {
                        Cell::from(multiline_cell)
                    }
                })
                .collect();

            app.max_heights[i] = *new_height.iter().max().unwrap_or(&1) as u16;
            Row::new(cells).height(app.current_heights[i])
        })
        .collect();

    app.rows = rows;
    app.max_heights = vec![1; app.rows.len()];
    app.current_heights = vec![1; app.rows.len()];

    let t = RatTable::new(app.rows.clone(), app.widths.clone())
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(select_style)
        .highlight_symbol("> ");

    f.render_stateful_widget(t, rects[0], &mut app.state);
    ui_input(f, &mut app.input);
}
