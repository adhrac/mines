use std::io::Write;

use crossterm::cursor::MoveToNextLine;
use crossterm::{ExecutableCommand, QueueableCommand, cursor, execute, terminal};
use crossterm::style::{Print, Color};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use crate::field::{Field, Cell, CellState, CellValue};
use Action::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const FIELD_TOP_LEFT_ROW: u16 = 0;
const FIELD_TOP_LEFT_COL: u16 = 0;

pub fn main() -> Result<()> {
    let mines = [
        (0,4), (0,7),
        (2,0), (2,1),
        (3,1), 
        (4,3),
        (5,3), (5,6),
        (7,0), (7,3),
    ];
    let field = Field::new_with_mines_at(8, 8, &mines);
    let mut t = TerminalApplication::new(std::io::stdout(), 8, 8);
    t.field = Some(field);

    t.open_application_window()?;
    t.field.as_mut().unwrap().reveal(2,4);
    t.print_field()?;

    loop {
        match await_input() {
            Quit => break,
            MoveLeft  => t.move_cursor(0, -1)?,
            MoveRight => t.move_cursor(0, 1)?,
            MoveUp    => t.move_cursor(-1, 0)?,
            MoveDown  => t.move_cursor(1, 0)?,
            Flag      => t.flag()?,
            Reveal    => t.reveal()?,
            a => println!("Not yet implemented: {a:?}"),
        }
        t.print_field()?;
    }

    t.close_application_window()?;
    Ok(())
}

struct TerminalApplication {
    w: std::io::Stdout,
    field: Option<Field>,
    rows: u16,
    cols: u16,
}

impl TerminalApplication {
    fn new(w: std::io::Stdout, rows: u16, cols: u16) -> Self {
        Self { w , field: None, rows, cols }
    }

    // Set up the application window which is an alternate-screen, raw-mode terminal.
    fn open_application_window(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.w, terminal::EnterAlternateScreen, cursor::MoveToRow(0))?;
        Ok(())
    }
    
    // Close the application window. If the program exits without doing this first, things will get weird.
    fn close_application_window(&mut self) -> Result<()> {
        execute!(self.w, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn terminal_style(cell: &Cell) -> char {
        todo!()
        // once you have figured out colors and background and stuff
    }

    fn print_field(&mut self) -> Result<()> {
        let (old_col, old_row) = cursor::position()?;
        execute!(self.w, cursor::MoveTo(FIELD_TOP_LEFT_COL, FIELD_TOP_LEFT_ROW))?;

        if let Some(field) = &mut self.field {
            for line in format!("{field}").lines() {
                self.w.queue(Print(line))?;
                self.w.queue(MoveToNextLine(1))?;
            }
        }
        else {
            for _ in 0..self.rows {
                for _ in 0..self.cols {
                    self.w.queue(Print("·"))?;
                }
                self.w.queue(MoveToNextLine(1))?;
            }
        }

        self.w.queue(cursor::MoveTo(old_col, old_row))?;
        self.w.flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, n_rows: i16, n_cols: i16) -> Result<()> {
        let (current_col, current_row) = cursor::position()?;
        let (current_col, current_row) = (current_col as i16, current_row as i16);

        let (mut new_col, mut new_row) = (current_col + n_cols, current_row + n_rows);

        if new_col < 0 {
            new_col = self.cols as i16 - 1;
        }
        if new_col > self.cols as i16 - 1 {
            new_col = 0;
        }

        if new_row < 0 {
            new_row = self.rows as i16 - 1;
        }
        if new_row > self.rows as i16 - 1 {
            new_row = 0;
        }

        self.w.execute(cursor::MoveTo(new_col as u16, new_row as u16))?;

        Ok(())
    }

    fn flag(&mut self) -> Result<()> {
        if let Some(field) = &mut self.field {
            let (cursor_col, cursor_row) = cursor::position()?;

            // coordinates on the field
            let field_row = (cursor_row - FIELD_TOP_LEFT_ROW) as usize;
            let field_col = (cursor_col - FIELD_TOP_LEFT_COL) as usize;

            match field.cells[field_row][field_col].state {
                CellState::Unflagged => field.flag(field_row, field_col),
                CellState::Flagged   => field.unflag(field_row, field_col),
                CellState::Revealed  => field.auto_flag(field_row, field_col),
            };
        }
        Ok(())
    }

    fn reveal(&mut self) -> Result<()> {
        if let Some(field) = &mut self.field {
            let (cursor_col, cursor_row) = cursor::position()?;

            // coordinates on the field
            let field_row = (cursor_row - FIELD_TOP_LEFT_ROW) as usize;
            let field_col = (cursor_col - FIELD_TOP_LEFT_COL) as usize;

            // reveal or autoreveal
            // they are allowed to blow themselves up
            match field.cells[field_row][field_col].state {
                CellState::Revealed => field.auto_reveal(field_row, field_col),
                _                   => field.reveal(field_row, field_col),
            };
        }
        else {
            // initialize
            todo!();
        }

        Ok(())
    }

}

impl Drop for TerminalApplication {
    fn drop(&mut self) {
        self.close_application_window().unwrap();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Action { MoveUp, MoveDown, MoveLeft, MoveRight, Reveal, Flag, Quit }

fn await_input() -> Action {
    use crossterm::event::KeyCode::*;
    use Action::*;

    loop {
        if let Ok(Event::Key(KeyEvent {
            code, // read the keycode of the keypress
            kind: KeyEventKind::Press, // makes sure we only capture keypresses
            modifiers: _,
            state: _,
        })) = event::read()
        {
            match code { // match on the keycode
                Char('d') => return Reveal,
                Char('f') => return Flag, 
                Char('h') => return MoveLeft, 
                Char('j') => return MoveDown, 
                Char('k') => return MoveUp, 
                Char('l') => return MoveRight, 
                Char('q') => return Quit,
                _ => (),
            }
        }
    }
}