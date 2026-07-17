use std::io::Write;

use super::Result;
use MenuAction::*;

use crossterm::{ExecutableCommand, QueueableCommand, cursor, terminal};
use crossterm::style::Print;

pub struct StartMenu {
    w: std::io::Stdout,
    current_selection: usize,
}

// (rows, cols, nr_mines)
const MENU_OPTIONS: [(u16, u16, usize); 3] = [(16, 30, 99), (16, 16, 40), (8, 8, 10)];

impl StartMenu {
    pub fn new(w: std::io::Stdout) -> Self {
        Self { w, current_selection: 0 }
    }

    pub fn get_start_options(&mut self) -> Result<(u16, u16, usize)> {
        self.draw_start_menu()?;

        loop {
            match await_input() {
                Confirm  => break,
                Quit     => return Err(Box::new(UserQuit)),
                MoveUp   => self.move_up()?,
                MoveDown => self.move_down()?,
                _        => (),
            }
            self.draw_start_menu()?;
        }

        self.w.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(MENU_OPTIONS[self.current_selection])
    }

    fn draw_start_menu(&mut self) -> Result<()> {
        self.w.queue(cursor::MoveTo(0,0))?;
        self.w.queue(Print("Select difficulty."))?;
        self.w.execute(cursor::MoveToNextLine(1))?;

        for (difficulty, (rows, cols, nr_mines)) in ["Hard", "Medium", "Easy"].into_iter().zip(MENU_OPTIONS) {
            self.w.queue(Print(format!(" [ ] - {difficulty} ({rows} x {cols}, {nr_mines} mines)")))?;
            self.w.queue(cursor::MoveToNextLine(2))?;
        }

        self.w.queue(cursor::MoveTo(2, 1 + self.current_selection as u16 * 2))?;
        self.w.queue(Print("-"))?;
        self.w.queue(cursor::MoveLeft(1))?;
        self.w.flush()?;
        Ok(())
    }

    fn move_up(&mut self) -> Result<()> {
        if self.current_selection == 0 {
            self.current_selection = MENU_OPTIONS.len() - 1;
        }
        else {
            self.current_selection -= 1;
        }

        Ok(())
    }

    fn move_down(&mut self) -> Result<()> {
        if self.current_selection == MENU_OPTIONS.len() - 1 {
            self.current_selection = 0;
        }
        else {
            self.current_selection += 1;
        }

        Ok(())
    }

    pub fn take_stdout(self) -> std::io::Stdout {
        self.w
    }
}

use crossterm::event::{Event, KeyEvent, KeyEventKind, self};

#[derive(Copy, Clone, Debug, PartialEq)]
enum MenuAction { MoveUp, MoveDown, MoveLeft, MoveRight, Confirm, Quit }

fn await_input() -> MenuAction {
    use crossterm::event::KeyCode::*;
    use MenuAction::*;

    loop {
        if let Ok(Event::Key(KeyEvent {
            code, // read the keycode of the keypress
            kind: KeyEventKind::Press, // makes sure we only capture keypresses
            modifiers: _,
            state: _,
        })) = event::read()
        {
            match code { // match on the keycode
                Char('d') | Enter => return Confirm,
                Char('h') | Left  => return MoveLeft, 
                Char('j') | Down  => return MoveDown, 
                Char('k') | Up    => return MoveUp, 
                Char('l') | Right => return MoveRight, 
                Char('q') => return Quit,
                #[cfg(debug_assertions)]
                Char('p') => panic!("User-initiated panic"),
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct UserQuit;

impl std::fmt::Display for UserQuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User quit.")
    }
}

impl std::error::Error for UserQuit {} 