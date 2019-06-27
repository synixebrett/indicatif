extern crate termion;

pub fn terminal_size() -> Option<(usize, usize)> {
    match termion::terminal_size() {
        Ok((cols, rows)) => Some((cols as usize, rows as usize)),
        Err(..) => None
    }
}

pub fn move_cursor_up(n: usize) -> String {
    format!("{}", termion::cursor::Up(n as u16))
}

pub fn show_cursor() {}
pub fn hide_cursor() {}
