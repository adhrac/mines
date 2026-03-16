use crossterm::{terminal, cursor, execute};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use crate::field::Field;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
    let mut t = TerminalApplication::new(std::io::stdout(), field);

    t.open_application_window()?;

    loop {
        let action = await_input();
        dbg!(action);
        execute!(t.w, cursor::MoveToColumn(0))?;
        if action == Action::Quit {
            break;
        }
    }

    t.close_application_window()?;
    Ok(())
}

struct TerminalApplication {
    w: std::io::Stdout,
    field: Option<Field>,
}

impl TerminalApplication {
    fn new(w: std::io::Stdout, field: Field) -> Self {
        Self { w , field: Some(field) }
    }


    // Set up the application window which is an alternate-screen, raw-mode terminal.
    pub fn open_application_window(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.w, terminal::EnterAlternateScreen, cursor::MoveToRow(0))?;
        Ok(())
    }
    
    // Close the application window. If the program exits without doing this first, things will get weird.
    pub fn close_application_window(&mut self) -> Result<()> {
        execute!(self.w, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
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