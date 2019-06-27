extern crate winapi;

/// Returns the size of the terminal, if available.
///
/// Note that this returns the size of the actual command window, and
/// not the overall size of the command window buffer
pub fn terminal_size() -> Option<(usize, usize)> {
    if let Some((_, csbi)) = get_csbi() {
        let w = (csbi.srWindow.Right - csbi.srWindow.Left) as usize;
        let h = (csbi.srWindow.Bottom - csbi.srWindow.Top) as usize;
        Some((w, h))
    } else {
        None
    }
}

/// move the cursor `n` lines up; return an empty string, just to
/// be aligned with the unix version.
pub fn move_cursor_up(n: usize) -> String {
    use self::winapi::um::wincon::{SetConsoleCursorPosition, COORD};
    if let Some((hand, csbi)) = get_csbi() {
        unsafe {
            SetConsoleCursorPosition(hand,
                                     COORD {
                                         X: 0,
                                         Y: csbi.dwCursorPosition.Y - n as i16,
                                     });
        }
    }
    "".to_string()
}

use self::winapi::um::wincon::{SetConsoleCursorInfo, CONSOLE_CURSOR_INFO};

pub fn hide_cursor() {
    if let Some((hand, _)) = get_csbi() {
        let hide: *const CONSOLE_CURSOR_INFO = &CONSOLE_CURSOR_INFO {
                                        dwSize: 1,
                                        bVisible: 0,
                                    };
        unsafe {
            SetConsoleCursorInfo(hand, hide);
        }
    }
}

pub fn show_cursor() {
    if let Some((hand, _)) = get_csbi() {
        let show: *const CONSOLE_CURSOR_INFO = &CONSOLE_CURSOR_INFO {
                                        dwSize: 1,
                                        bVisible: 1,
                                    };
        unsafe {
            SetConsoleCursorInfo(hand, show);
        }
    }
}

fn get_csbi() -> Option<(self::winapi::shared::ntdef::HANDLE, self::winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO)> {
    use self::winapi::shared::ntdef::HANDLE;
    use self::winapi::um::processenv::GetStdHandle;
    use self::winapi::um::winbase::STD_OUTPUT_HANDLE;
    use self::winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

    let hand: HANDLE = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let zc = COORD { X: 0, Y: 0 };
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: zc.clone(),
        dwCursorPosition: zc.clone(),
        wAttributes: 0,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: zc,
    };
    match unsafe { GetConsoleScreenBufferInfo(hand, &mut csbi) } {
        0 => None,
        _ => Some((hand, csbi)),
    }
}
