use std::io::{self, Read, StdinLock};
use std::os::unix::io::AsRawFd;

mod screen;
mod terminal;
use terminal::Terminal;

const fn ctrl_key(val: u8) -> u8 {
    return val & 0b00011111;
}

const CTRL_Q: u8 = ctrl_key(b'q');

fn is_cntrl(c: u8) -> bool {
    if c < 32 || c == 127 {
        return true;
    }
    return false;
}

fn read_loop(stdin: &mut StdinLock) -> io::Result<()> {
    let mut buf: [u8; 4];

    loop {
        buf = [0, 0, 0, 0];

        screen::refresh()?;
        screen::reset_cursor()?;

        stdin.read(&mut buf)?;

        match buf[0] {
            CTRL_Q => break,
            c => {
                if !is_cntrl(c) {
                    print!("{} ({})\r\n", c, c as char)
                } else {
                    print!("{:?}\r\n", buf);
                }
            }
        }

        screen::draw_rows()?;
        screen::reset_cursor()?;
    }

    return Ok(());
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let terminal = Terminal::new(handle.as_raw_fd());

    terminal.enable_raw_mode();
    match read_loop(&mut handle) {
        _ => {
            screen::refresh()?;
            screen::reset_cursor()?;
        }
    }
    terminal.disable_raw_mode();

    return Ok(());
}
