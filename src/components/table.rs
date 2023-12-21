use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
use std::io;

pub struct App<'a> {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
    pub max_heights: Vec<u16>,
    pub current_heights: Vec<u16>,
    pub rows: Vec<Row<'a>>,
}

impl<'a> App<'a> {
    pub fn new(items: Vec<Vec<String>>) -> App<'a> {
        App {
            state: TableState::default(),
            max_heights: vec![1; items.len()],
            current_heights: vec![1; items.len()],
            items,
            rows: Vec::new(),
        }
    }

    pub fn next(&mut self, count: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - count {
                    self.current_heights[i - count] = 1;
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
                    self.items.len() - count
                } else {
                    i - count
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn expand(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if self.current_heights[i] == 1 {
                    self.current_heights[i] = self.max_heights[i];
                } else {
                    self.current_heights[i] = 1;
                }
            }
            None => (),
        };
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(1),
                KeyCode::Up => app.previous(1),
                KeyCode::PageDown => app.next(10),
                KeyCode::PageUp => app.previous(10),
                KeyCode::Enter => app.expand(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let select_style = Style::default().add_modifier(Modifier::REVERSED);

    let header = Row::new(
        [
            "ID",
            "Package name",
            "Version",
            "Description",
            "Licenses",
            "URL",
            "Installed Size",
            "Install Date",
        ]
        .iter()
        .map(|h| {
            Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        }),
    )
    .height(1)
    .bottom_margin(1);

    let widths = [
        Constraint::Percentage(2),
        Constraint::Percentage(12),
        Constraint::Percentage(14),
        Constraint::Percentage(23),
        Constraint::Percentage(14),
        Constraint::Percentage(14),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
    ];

    let rows: Vec<Row<'_>> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let mut new_height = vec![];

            let cells: Vec<Cell> = item
                .iter()
                .enumerate()
                .map(|(j, &ref c)| {
                    let height = widths[j].apply(f.size().width);
                    let binding = c.chars().collect::<Vec<_>>();
                    let vec = binding.chunks(height as usize);

                    let cell_content = vec
                        .clone()
                        .map(|v| v.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join("\n");

                    new_height.push(vec.len());

                    if j == 1 {
                        Cell::from(c.to_string()).bold()
                    } else if j == 4 {
                        Cell::from(
                            Line::styled(cell_content, Style::default().green())
                                .alignment(ratatui::layout::Alignment::Center),
                        )
                    } else {
                        Cell::from(cell_content)
                    }
                })
                .collect();

            app.max_heights[i] = *new_height.iter().max().unwrap_or(&1) as u16;
            Row::new(cells).height(app.current_heights[i])
        })
        .collect();

    app.rows = rows;

    let t = Table::new(app.rows.clone(), widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("System Package List"),
        )
        .highlight_style(select_style)
        .highlight_symbol("* ");

    f.render_stateful_widget(t, rects[0], &mut app.state);
}
