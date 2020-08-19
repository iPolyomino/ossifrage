use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    Result,
};

pub fn read_char() -> Result<char> {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = event::read()?
        {
            return Ok(c);
        }
    }
}
