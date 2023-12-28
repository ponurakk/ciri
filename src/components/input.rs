use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct Input {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    cursor_position: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input value
    pub message: String,
    /// Size and position of input
    pub size: Option<Box<dyn Fn(Rect) -> Rect>>,
}

impl Input {
    pub fn new(size: Option<Box<dyn Fn(Rect) -> Rect>>) -> Input {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            message: String::new(),
            cursor_position: 0,
            size,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn submit_message(&mut self) {
        self.message = self.input.clone();
        // TODO: Importand for live filter
        // self.input.clear();
        // self.reset_cursor();
    }
}

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut Input) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

pub fn ui(f: &mut Frame, app: &Input) {
    let chunks = if let Some(size) = &app.size {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(size(f.size()))
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(f.size())
    };

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Blue),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[0]);

    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                chunks[0].x + app.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[0].y + 1,
            )
        }
    }
}
