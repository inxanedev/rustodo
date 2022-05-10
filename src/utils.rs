use std::io::{stdout, stdin, Write};

use crossterm::{terminal::{Clear, ClearType}, ExecutableCommand, cursor::MoveTo};

#[inline]
pub fn clear() {
    stdout().execute(Clear(ClearType::All)).unwrap();
    stdout().execute(MoveTo(0, 0)).unwrap();
}

pub fn get_input() -> String {
    let mut i = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut i).unwrap();
    i.trim().to_owned()
}
