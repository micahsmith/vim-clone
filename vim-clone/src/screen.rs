use std::io::{self, Write};

pub fn reset_cursor() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(b"\x1b[H")?;
    return Ok(());
}

pub fn refresh() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(b"\x1b[2J")?;
    return Ok(());
}

pub fn draw_rows() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for _ in 0..24 {
        handle.write(b"~\r\n")?;
    }
    return Ok(());
}
